use crate::battle::action::{BattleAction, BattleActionKind};
use crate::battle::config::BattleConfig;
use crate::battle::error::BattleError;
use crate::battle::event::BattleEvent;
use crate::battle::pokemon::BattlePokemon;
use crate::battle::side::BattleSide;
use crate::battle::target::{BattleTarget, BattleTargetSingle};
use crate::battle::team::BattleTeam;
use crate::data::Data;
use crate::error::PkmnResult;
use crate::types::move_damage_class::MoveDamageClass;
use crate::types::move_target::MoveTarget;
use std::sync::Arc;

pub mod action;
mod builder;
pub mod config;
pub mod error;
pub mod event;
pub mod pokemon;
pub mod side;
pub mod slot;
pub mod target;
pub mod team;

pub struct Battle {
    data: Arc<Data>,
    config: BattleConfig,
    side_a: Vec<BattleTeam>,
    side_b: Vec<BattleTeam>,
    actions: Vec<BattleAction>,
    action_priorities: Vec<isize>,
    rng: Box<dyn rand::Rng>,
}

impl Battle {
    pub fn builder(data: &Arc<Data>) -> builder::BattleBuilder {
        builder::BattleBuilder::new(data)
    }
}

// Actions
impl Battle {
    pub fn queue_action(&mut self, action: BattleAction) -> PkmnResult<()> {
        if !self.target_exists(BattleTarget::Single(action.source)) {
            return Err(BattleError::InvalidActionSource(action.source).into());
        };

        let already_taken = self.actions.iter().any(|a| a.source == action.source);
        if already_taken {
            return Err(BattleError::ActionAlreadyTaken(action.source).into());
        }

        let priority = self.action_priority(&action)?;
        self.actions.push(action);
        self.action_priorities.push(priority);

        Ok(())
    }

    fn action_priority(&self, action: &BattleAction) -> PkmnResult<isize> {
        let base_priority = match action.kind {
            BattleActionKind::UseMove { move_index, .. } => {
                let Some(source) = self.get_target(action.source) else {
                    // ToDo: Handle properly
                    return Ok(0);
                };
                let Some(stored_move) = source.get_move(move_index) else {
                    // ToDo: Handle properly
                    return Ok(0);
                };
                let move_ = self.data.get_move(stored_move.move_id)?;
                move_.priority as isize
            }
        };
        Ok(base_priority)
    }

    fn sort_actions(&mut self) {
        let mut indices: Vec<usize> = (0..self.actions.len()).collect();
        indices.sort_by(|&a, &b| self.action_priorities[b].cmp(&self.action_priorities[a]));

        let old_actions = std::mem::take(&mut self.actions);
        let old_priorities = std::mem::take(&mut self.action_priorities);

        self.actions = indices.iter().map(|&i| old_actions[i].clone()).collect();
        self.action_priorities = indices.iter().map(|&i| old_priorities[i]).collect();
    }

    fn take_actions(&mut self) -> Vec<BattleAction> {
        self.sort_actions();
        self.action_priorities.clear();
        std::mem::take(&mut self.actions)
    }

    fn execute_action(
        &mut self,
        events: &mut Vec<BattleEvent>,
        action: &BattleAction,
    ) -> PkmnResult<()> {
        match action.kind {
            BattleActionKind::UseMove { move_index, target } => {
                self.execute_move(action.source, target, move_index, events)?;
            }
        }
        Ok(())
    }
}

// Turn resolution
impl Battle {
    pub fn resolve_turn(&mut self) -> PkmnResult<Vec<BattleEvent>> {
        if !self.turn_ready() {
            return Err(BattleError::TurnNotReady.into());
        }

        let mut events = vec![];
        for action in self.take_actions() {
            if !self.target_alive(action.source) {
                continue;
            }
            self.execute_action(&mut events, &action)?;
        }

        Ok(events)
    }
}

// Access helpers
impl Battle {
    pub fn get_target(&self, target: BattleTargetSingle) -> Option<&BattlePokemon> {
        let team_index = target.slot / self.config.active_slots_per_team;

        let team = match target.side {
            BattleSide::A => self.side_a.get(team_index),
            BattleSide::B => self.side_b.get(team_index),
        }?;

        let slot_index = target.slot % self.config.active_slots_per_team;
        team.get_active_pokemon(slot_index)
    }

    pub fn get_target_mut(&mut self, target: BattleTargetSingle) -> Option<&mut BattlePokemon> {
        let team_index = target.slot / self.config.active_slots_per_team;

        let team = match target.side {
            BattleSide::A => self.side_a.get_mut(team_index),
            BattleSide::B => self.side_b.get_mut(team_index),
        }?;

        let slot_index = target.slot % self.config.active_slots_per_team;
        team.get_active_pokemon_mut(slot_index)
    }
}

// Check helpers
impl Battle {
    pub fn target_alive(&self, target: BattleTargetSingle) -> bool {
        let Some(pokemon) = self.get_target(target) else {
            return false;
        };
        pokemon.is_alive()
    }

    pub fn target_exists(&self, target: BattleTarget) -> bool {
        match target {
            BattleTarget::Single(single) => single.slot < self.config.active_slots_per_team,
        }
    }

    /// Checks if the current turn is ready to be executed (all actions have been queued)
    pub fn turn_ready(&self) -> bool {
        self.actions.len() == self.config.active_slots_per_team * 2
    }
}

// Move execution
impl Battle {
    fn execute_move(
        &mut self,
        source: BattleTargetSingle,
        target: BattleTargetSingle,
        move_index: usize,
        events: &mut Vec<BattleEvent>,
    ) -> PkmnResult<()> {
        let Some(source_pokemon) = self.get_target(source) else {
            // ToDo: Handle properly
            return Ok(());
        };
        let Some(stored_move) = source_pokemon.get_move(move_index) else {
            // ToDo: Handle properly
            return Ok(());
        };
        let move_id = stored_move.move_id;

        let m = self.data.get_move(move_id)?;
        let power = m.power;
        let damage_class = m.damage_class;
        let move_target = m.target;

        events.push(BattleEvent::MoveAnnounced { source, move_id });

        // ToDo: Accuracy check

        let targets = self.resolve_targets(target, move_target);
        for target in targets {
            if !self.target_alive(target) {
                continue;
            }

            if damage_class != MoveDamageClass::Status {
                // ToDo: Actual damage calculation
                let damage = power as u16;
                let damage_dealt = self.apply_damage(target, damage);

                events.push(BattleEvent::Damage {
                    target,
                    damage: damage_dealt,
                });

                if !self.target_alive(target) {
                    events.push(BattleEvent::Fainted { target });
                }
            }
        }

        Ok(())
    }

    fn resolve_targets(
        &self,
        target: BattleTargetSingle,
        _move_target: MoveTarget,
    ) -> Vec<BattleTargetSingle> {
        // ToDo: Properly resolve targets
        vec![target]
    }

    fn apply_damage(&mut self, target: BattleTargetSingle, damage: u16) -> u16 {
        // ToDo: Handle target not existing or not being alive anymore properly
        let Some(pokemon) = self.get_target_mut(target) else {
            return 0;
        };
        pokemon.apply_damage(damage)
    }
}

// RNG
impl Battle {
    pub fn rng(&mut self) -> &mut dyn rand::Rng {
        &mut *self.rng
    }
}
