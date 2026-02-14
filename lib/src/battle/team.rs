use crate::battle::slot::BattleSlot;

pub struct BattleTeam {
    active: Vec<BattleSlot>,
    pokemon: [BattleSlot; 6],
}
