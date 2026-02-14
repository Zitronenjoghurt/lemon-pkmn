use strum_macros::{Display, EnumIter, FromRepr};

/// Source: https://github.com/PokeAPI/pokeapi/blob/master/data/v2/csv/types.csv
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter, Display, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum PokemonType {
    Normal = 1,
    Fighting = 2,
    Flying = 3,
    Poison = 4,
    Ground = 5,
    Rock = 6,
    Bug = 7,
    Ghost = 8,
    Steel = 9,
    Fire = 10,
    Water = 11,
    Grass = 12,
    Electric = 13,
    Psychic = 14,
    Ice = 15,
    Dragon = 16,
    Dark = 17,
    Fairy = 18,
    Stellar = 19,
    Unknown = 10001,
    Shadow = 10002,
}
