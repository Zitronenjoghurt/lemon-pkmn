use crate::battle::types::event::BattleEvent;
use crate::battle::types::target::BattleTargetSingle;
use crate::battle::Battle;
use crate::data::move_id::MoveId;
use crate::data::MoveData;
use crate::error::PkmnResult;
use crate::types::move_target::MoveTarget;
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

            // ToDo: Proper damage calculation
            let damage = self.move_power()? as u16;
            let Some(dealt) = self.battle.apply_damage(self.target, damage) else {
                break;
            };

            self.battle.push_event(BattleEvent::Damage {
                target: self.target,
                damage: dealt,
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
