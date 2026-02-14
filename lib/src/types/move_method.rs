use strum_macros::{Display, EnumIter, FromRepr};

/// Source: https://github.com/PokeAPI/pokeapi/blob/6554554bb35a9e6a62ae5114eef83584f994666d/data/v2/csv/pokemon_move_methods.csv
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, FromRepr)]
#[repr(u8)]
pub enum MoveMethod {
    LevelUp = 1,
    Egg = 2,
    Tutor = 3,
    Machine = 4,
    StadiumSurfingPikachu = 5,
    LightBallEgg = 6,
    ColosseumPurification = 7,
    XdShadow = 8,
    XdPurification = 9,
    FormChange = 10,
    ZygardeCube = 11,
}
