use crate::battle::participant::BattleParticipant;

mod action;
mod event;
mod participant;
mod pokemon;
mod side;
mod slot;
mod target;
mod team;

pub struct Battle {
    side_a: Vec<BattleParticipant>,
    side_b: Vec<BattleParticipant>,
    slots_per_participant: usize,
}
