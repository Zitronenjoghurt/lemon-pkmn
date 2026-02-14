use crate::battle::pokemon::BattlePokemon;

pub enum BattleSlot {
    Filled(BattlePokemon),
    Empty,
}
