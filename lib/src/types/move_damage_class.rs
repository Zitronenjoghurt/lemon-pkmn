use strum_macros::{Display, EnumIter, FromRepr};

/// Source: https://github.com/PokeAPI/pokeapi/blob/master/data/v2/csv/move_damage_classes.csv
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter, Display, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum MoveDamageClass {
    Status = 1,
    Physical = 2,
    Special = 3,
}
