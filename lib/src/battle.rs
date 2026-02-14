use crate::battle::team::BattleTeam;

mod action;
mod event;
mod pokemon;
mod side;
mod slot;
mod target;
mod team;

pub struct Battle {
    side_a: Vec<BattleTeam>,
    side_b: Vec<BattleTeam>,
    slots_per_participant: usize,
}
