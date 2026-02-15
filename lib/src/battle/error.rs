use crate::battle::types::target::BattleTargetSingle;

pub type BattleResult<T> = Result<T, BattleError>;

#[derive(Debug, thiserror::Error)]
pub enum BattleError {
    #[error("Pokémon has already taken its action this turn: {0}")]
    ActionAlreadyTaken(BattleTargetSingle),
    #[error("There is no Pokémon at the specified action source: {0}")]
    InvalidActionSource(BattleTargetSingle),
    #[error("There is no Pokémon at the specified action target: {0}")]
    InvalidActionTarget(BattleTargetSingle),
    #[error("Pokémon at position {user} does not have a move in slot {slot}")]
    NoMoveInSlot {
        user: BattleTargetSingle,
        slot: usize,
    },
    #[error("Turn is not ready yet, missing actions")]
    TurnNotReady,
}
