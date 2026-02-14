use crate::battle::error::BattleError;

pub type PkmnResult<T> = Result<T, PkmnError>;

#[derive(Debug, thiserror::Error)]
pub enum PkmnError {
    #[error("Battle Error: {0}")]
    Battle(#[from] BattleError),
    #[cfg(feature = "bitcode")]
    #[error("Bitcode Error: {0}")]
    Bitcode(#[from] bitcode::Error),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Move not found: {0}")]
    MoveNotFound(u16),
    #[error("Species not found: {0}")]
    SpeciesNotFound(u16),
}
