use crate::battle::config::BattleConfig;
use crate::battle::error::BattleError;
use crate::battle::sim::move_executor::MoveExecutor;
use crate::data::move_pipeline::move_pipeline;
use crate::data::{Data, SpeciesData};
use crate::error::PkmnResult;
use std::sync::Arc;
use types::action::{BattleAction, BattleActionKind};
use types::event::BattleEvent;
use types::pokemon::BattlePokemon;
use types::side::BattleSide;
use types::target::BattleTargetSingle;
use types::team::BattleTeam;

mod builder;
pub mod config;
pub mod error;
pub mod sim;
pub mod types;

pub struct Battle {
    data: Arc<Data>,
    config: BattleConfig,
    side_a: Vec<BattleTeam>,
    side_b: Vec<BattleTeam>,
    actions: Vec<BattleAction>,
    action_priorities: Vec<isize>,
    events: Vec<BattleEvent>,
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
        let Some(source) = self.get_target(action.source) else {
            return Err(BattleError::InvalidActionSource(action.source).into());
        };

        let already_taken = self.actions.iter().any(|a| a.source == action.source);
        if already_taken {
            return Err(BattleError::ActionAlreadyTaken(action.source).into());
        }

        if let BattleActionKind::UseMove { move_index, target } = action.kind {
            if !self.target_exists(target) {
                return Err(BattleError::InvalidActionTarget(target).into());
            };

            if source.get_move(move_index).is_none() {
                return Err(BattleError::NoMoveInSlot {
                    user: action.source,
                    slot: move_index,
                }
                .into());
            };
        }

        // ToDo: Check remaining PP

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

    fn execute_action(&mut self, action: &BattleAction) -> PkmnResult<()> {
        match action.kind {
            BattleActionKind::UseMove { move_index, target } => {
                self.execute_move(action.source, target, move_index)?;
            }
        }
        Ok(())
    }
}

// Events
impl Battle {
    pub fn push_event(&mut self, event: BattleEvent) {
        self.events.push(event);
    }

    pub fn take_events(&mut self) -> Vec<BattleEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn debug_message(&mut self, message: impl Into<String>) {
        self.push_event(BattleEvent::debug(message));
    }

    pub fn info_message(&mut self, message: impl Into<String>) {
        self.push_event(BattleEvent::info(message));
    }
}

// Turn resolution
impl Battle {
    pub fn resolve_turn(&mut self) -> PkmnResult<Vec<BattleEvent>> {
        if !self.turn_ready() {
            return Err(BattleError::TurnNotReady.into());
        }

        for action in self.take_actions() {
            if !self.target_alive(action.source) {
                continue;
            }
            self.execute_action(&action)?;
        }

        // ToDo: Pokemon with no PP left should struggle

        Ok(self.take_events())
    }
}

// Move execution
impl Battle {
    fn execute_move(
        &mut self,
        source: BattleTargetSingle,
        target: BattleTargetSingle,
        move_index: usize,
    ) -> PkmnResult<()> {
        let Some(source_pokemon) = self.get_target(source) else {
            // ToDo: Handle properly, might have to change target, etc.
            return Ok(());
        };
        let Some(stored_move) = source_pokemon.get_move(move_index) else {
            return Err(BattleError::NoMoveInSlot {
                user: source,
                slot: move_index,
            }
            .into());
        };

        let move_id = stored_move.move_id;
        let pipeline = move_pipeline(move_id);
        MoveExecutor::new(self, move_id, source, target).run(pipeline)?;

        Ok(())
    }

    fn apply_damage(&mut self, target: BattleTargetSingle, damage: u16) -> Option<u16> {
        Some(self.get_target_mut(target)?.apply_damage(damage))
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

    pub fn get_target_species_data(&self, target: BattleTargetSingle) -> Option<&SpeciesData> {
        self.data
            .get_species(self.get_target(target)?.base.species_id)
            .ok()
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

    pub fn target_exists(&self, target: BattleTargetSingle) -> bool {
        self.get_target(target).is_some()
    }

    /// Checks if the current turn is ready to be executed (all actions have been queued)
    pub fn turn_ready(&self) -> bool {
        // ToDo: Only check for actions of active pokemon
        self.actions.len() == self.config.active_slots_per_team * 2
    }
}

// RNG
impl Battle {
    pub fn rng(&mut self) -> &mut dyn rand::Rng {
        &mut *self.rng
    }
}
