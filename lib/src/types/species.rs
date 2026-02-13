use strum_macros::{Display, EnumIter};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, EnumIter, Display)]
#[repr(u16)]
pub enum SpeciesId {
    Bulbasaur = 1,
}
