use crate::data::moves::MoveId;
use crate::data::species::SpeciesId;

pub type PkmnResult<T> = Result<T, PkmnError>;

#[derive(Debug, thiserror::Error)]
pub enum PkmnError {
    #[cfg(feature = "bitcode")]
    #[error("Bitcode Error: {0}")]
    Bitcode(#[from] bitcode::Error),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Move not found: {0}")]
    MoveNotFound(MoveId),
    #[error("Species not found: {0}")]
    SpeciesNotFound(SpeciesId),
}
