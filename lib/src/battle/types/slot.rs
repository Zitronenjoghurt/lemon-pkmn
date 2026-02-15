use crate::battle::types::pokemon::BattlePokemon;

#[derive(Debug, Default)]
pub enum BattleSlot {
    Filled(BattlePokemon),
    #[default]
    Empty,
}
