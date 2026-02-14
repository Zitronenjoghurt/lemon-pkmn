use strum_macros::{Display, EnumIter, FromRepr};

/// Source: https://github.com/PokeAPI/pokeapi/blob/master/data/v2/csv/move_damage_classes.csv
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    bitcode::Encode,
    bitcode::Decode,
    serde::Serialize,
    serde::Deserialize,
    EnumIter,
    Display,
    FromRepr,
)]
#[repr(u8)]
pub enum MoveDamageClass {
    Status = 1,
    Physical = 2,
    Special = 3,
}
