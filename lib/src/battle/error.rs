use crate::battle::target::BattleTargetSingle;

pub type BattleResult<T> = Result<T, BattleError>;

#[derive(Debug, thiserror::Error)]
pub enum BattleError {
    #[error("Pokémon has already taken its action this turn: {0}")]
    ActionAlreadyTaken(BattleTargetSingle),
    #[error("There is no Pokémon at the specified action source: {0}")]
    InvalidActionSource(BattleTargetSingle),
    #[error("Turn is not ready yet, missing actions")]
    TurnNotReady,
}
