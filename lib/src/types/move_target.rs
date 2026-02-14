use strum_macros::{Display, EnumIter, FromRepr};

/// Source: https://github.com/PokeAPI/pokeapi/blob/master/data/v2/csv/move_targets.csv
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
pub enum MoveTarget {
    MoveSpecific = 1,
    SelectedPokemonMeFirst = 2,
    Ally = 3,
    UserField = 4,
    UserOrAlly = 5,
    OpponentsField = 6,
    User = 7,
    RandomOpponent = 8,
    AllOtherPokemon = 9,
    SelectedPokemon = 10,
    AllOpponents = 11,
    EntireField = 12,
    UserAndAllies = 13,
    AllPokemon = 14,
    AllAllies = 15,
    FaintingPokemon = 16,
}
