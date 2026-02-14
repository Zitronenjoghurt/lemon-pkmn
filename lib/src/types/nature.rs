use crate::types::stats::Stat;
use strum_macros::{Display, EnumIter, FromRepr};

/// Sources:
/// - https://github.com/PokeAPI/pokeapi/blob/master/data/v2/csv/natures.csv
/// - https://bulbapedia.bulbagarden.net/wiki/Nature
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, FromRepr)]
#[repr(u8)]
pub enum Nature {
    Hardy = 1,
    Bold = 2,
    Modest = 3,
    Calm = 4,
    Timid = 5,
    Lonely = 6,
    Docile = 7,
    Mild = 8,
    Gentle = 9,
    Hasty = 10,
    Adamant = 11,
    Impish = 12,
    Bashful = 13,
    Careful = 14,
    Rash = 15,
    Jolly = 16,
    Naughty = 17,
    Lax = 18,
    Quirky = 19,
    Naive = 20,
    Brave = 21,
    Relaxed = 22,
    Quiet = 23,
    Sassy = 24,
    Serious = 25,
}

impl Nature {
    pub fn increased_stat(&self) -> Stat {
        match self {
            Self::Lonely | Self::Brave | Self::Adamant | Self::Naughty | Self::Hardy => Stat::Atk,
            Self::Bold | Self::Relaxed | Self::Impish | Self::Lax | Self::Docile => Stat::Def,
            Self::Modest | Self::Mild | Self::Quiet | Self::Rash | Self::Bashful => Stat::SpAtk,
            Self::Calm | Self::Gentle | Self::Careful | Self::Sassy | Self::Quirky => Stat::SpDef,
            Self::Timid | Self::Hasty | Self::Jolly | Self::Naive | Self::Serious => Stat::Speed,
        }
    }

    pub fn decreased_stat(&self) -> Stat {
        match self {
            Self::Bold | Self::Modest | Self::Calm | Self::Timid | Self::Hardy => Stat::Atk,
            Self::Lonely | Self::Mild | Self::Gentle | Self::Hasty | Self::Docile => Stat::Def,
            Self::Adamant | Self::Impish | Self::Careful | Self::Jolly | Self::Bashful => {
                Stat::SpAtk
            }
            Self::Naughty | Self::Lax | Self::Rash | Self::Naive | Self::Quirky => Stat::SpDef,
            Self::Brave | Self::Relaxed | Self::Quiet | Self::Sassy | Self::Serious => Stat::Speed,
        }
    }

    pub fn is_neutral(&self) -> bool {
        self.increased_stat() == self.decreased_stat()
    }
}
