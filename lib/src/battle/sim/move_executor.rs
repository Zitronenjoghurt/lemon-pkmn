use crate::battle::error::BattleError;
use crate::battle::types::event::BattleEvent;
use crate::battle::types::target::BattleTargetSingle;
use crate::battle::types::type_effectiveness::TypeEffectiveness;
use crate::battle::Battle;
use crate::data::move_id::MoveId;
use crate::data::MoveData;
use crate::error::{PkmnError, PkmnResult};
use crate::types::move_damage_class::MoveDamageClass;
use crate::types::move_target::MoveTarget;
use crate::types::pokemon_type::PokemonType;
use rand::RngExt;

#[derive(Debug, Copy, Clone)]
pub enum MoveStep {
    NotImplemented,
    Announce,
    AccuracyCheck,
    MultiHit { min: u8, max: u8 },
    DealDamage,
}

pub struct MoveExecutor<'a> {
    pub battle: &'a mut Battle,
    pub move_id: MoveId,
    pub source: BattleTargetSingle,
    pub target: BattleTargetSingle,
    pub missed: bool,
    pub hit_count: u8,
}

impl<'a> MoveExecutor<'a> {
    pub fn new(
        battle: &'a mut Battle,
        move_id: MoveId,
        source: BattleTargetSingle,
        target: BattleTargetSingle,
    ) -> Self {
        Self {
            battle,
            move_id,
            source,
            target,
            missed: false,
            hit_count: 1,
        }
    }

    pub fn run(&mut self, steps: &[MoveStep]) -> PkmnResult<()> {
        for step in steps {
            if self.missed {
                break;
            }
            self.execute(*step)?;
        }
        Ok(())
    }

    fn execute(&mut self, step: MoveStep) -> PkmnResult<()> {
        match step {
            MoveStep::NotImplemented => {
                self.battle
                    .info_message(format!("Move {} is not implemented yet", self.move_id));
            }
            MoveStep::Announce => self.announce_move(),
            MoveStep::AccuracyCheck => self.accuracy_check()?,
            MoveStep::MultiHit { min, max } => self.multi_hit(min, max),
            MoveStep::DealDamage => self.deal_damage()?,
        }
        Ok(())
    }
}

// Data Helpers
impl MoveExecutor<'_> {
    pub fn move_data(&self) -> PkmnResult<&MoveData> {
        self.battle.data.get_move(self.move_id)
    }

    pub fn move_power(&self) -> PkmnResult<u8> {
        Ok(self.move_data()?.power)
    }

    pub fn move_accuracy(&self) -> PkmnResult<u8> {
        Ok(self.move_data()?.accuracy)
    }

    pub fn move_target(&self) -> PkmnResult<MoveTarget> {
        Ok(self.move_data()?.target)
    }

    pub fn move_damage_class(&self) -> PkmnResult<MoveDamageClass> {
        Ok(self.move_data()?.damage_class)
    }

    pub fn move_type(&self) -> PkmnResult<PokemonType> {
        Ok(self.move_data()?.pokemon_type)
    }
}

// Step execution
impl MoveExecutor<'_> {
    pub fn announce_move(&mut self) {
        self.battle.push_event(BattleEvent::MoveAnnounced {
            source: self.source,
            move_id: self.move_id,
        });
    }

    pub fn accuracy_check(&mut self) -> PkmnResult<()> {
        let base_accuracy = self.move_accuracy()?;

        if base_accuracy == 0 {
            self.battle
                .debug_message("Move has 0 accuracy, skipping accuracy check");
            return Ok(());
        };

        // TODO: apply accuracy/evasion stat stages
        // TODO: apply ability modifiers (Compound Eyes, Sand Veil, etc.)
        // TODO: apply item modifiers (Wide Lens, Bright Powder, etc.)
        let accuracy = base_accuracy as u16;

        let roll = self.battle.rng.random_range(0..100u16);
        self.battle.debug_message(format!(
            "Accuracy check | (Base/Effective): {base_accuracy}/{accuracy} | Roll: {roll}"
        ));

        if roll >= accuracy {
            self.missed = true;
            self.battle.push_event(BattleEvent::MoveMissed {
                source: self.source,
            });
        };

        Ok(())
    }

    pub fn multi_hit(&mut self, min: u8, max: u8) {
        self.hit_count = self.battle.rng.random_range(min..=max);
    }

    pub fn deal_damage(&mut self) -> PkmnResult<()> {
        // ToDo: Resolve battle target from move target
        for _ in 0..self.hit_count {
            if !self.battle.target_alive(self.target) {
                break;
            }

            let (damage, effectiveness) = self.calculate_damage(self.target)?;
            let Some(dealt) = self.battle.apply_damage(self.target, damage) else {
                break;
            };

            self.battle.push_event(BattleEvent::Damage {
                target: self.target,
                damage: dealt,
                effectiveness,
            });

            if !self.battle.target_alive(self.target) {
                self.battle.push_event(BattleEvent::Fainted {
                    target: self.target,
                });
            }
        }

        Ok(())
    }
}

// Calculation
impl MoveExecutor<'_> {
    /// Source: https://bulbapedia.bulbagarden.net/wiki/Damage#Generation_V_onward
    fn calculate_damage(
        &mut self,
        target: BattleTargetSingle,
    ) -> PkmnResult<(u16, TypeEffectiveness)> {
        let damage_class = self.move_damage_class()?;
        if damage_class == MoveDamageClass::Status {
            return Ok((0, TypeEffectiveness::Normal));
        };
        let Some(source_pokemon) = self.battle.get_target(self.source) else {
            return Err(BattleError::InvalidActionSource(self.source).into());
        };
        let Some(target_pokemon) = self.battle.get_target(target) else {
            return Err(BattleError::InvalidActionTarget(target).into());
        };
        let Some(source_data) = self.battle.get_target_species_data(self.source) else {
            return Err(PkmnError::SpeciesNotFound(
                source_pokemon.base.species_id as u16,
            ));
        };

        let level = source_pokemon.base.level as u32;

        let power = self.move_power()? as u32;

        let (atk, def) = if damage_class == MoveDamageClass::Physical {
            (source_pokemon.stats.atk, target_pokemon.stats.def)
        } else {
            (source_pokemon.stats.sp_atk, target_pokemon.stats.sp_def)
        };

        let stab = if source_data.has_type(self.move_type()?) {
            1.5
        } else {
            1.0
        };

        let random = self.battle.rng.random_range(85.0..=100.0) / 100.0;

        let type_effect_factor = self.target_type_effectiveness(target, self.move_type()?)?;
        if type_effect_factor == 0.0 {
            return Ok((0, TypeEffectiveness::Immune));
        }

        let effectiveness = if type_effect_factor > 1.0 {
            TypeEffectiveness::Effective
        } else if type_effect_factor == 1.0 {
            TypeEffectiveness::Normal
        } else {
            TypeEffectiveness::NotEffective
        };

        // ToDo: Critical hits
        // ToDo: Targets
        // ToDo: Parental Bond
        // ToDo: Weather
        // ToDo: GlaiveRush
        // ToDo: Burn
        // ToDo: Z-Move
        // ToDo: Terastallization

        let base_damage = ((((((2 * level) as f64 / 5.0).floor() as u32 + 2)
            * power
            * (atk as f64 / def as f64).floor() as u32) as f64)
            / 50.0)
            .floor() as u16;
        let damage = ((base_damage as f64 * stab * random).floor() as u16).max(1);

        Ok((damage, effectiveness))
    }

    fn target_type_effectiveness(
        &self,
        target: BattleTargetSingle,
        attacking_type: PokemonType,
    ) -> PkmnResult<f64> {
        let Some(target_data) = self.battle.get_target_species_data(target) else {
            return Err(BattleError::InvalidActionTarget(target).into());
        };
        Ok(attacking_type.effectiveness(target_data.primary_type, target_data.secondary_type))
    }
}
