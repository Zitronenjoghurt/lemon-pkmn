use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, Display, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum VersionGroup {
    RedBlue = 1,
    Yellow = 2,
    GoldSilver = 3,
    Crystal = 4,
    RubySapphire = 5,
    Emerald = 6,
    FireredLeafgreen = 7,
    DiamondPearl = 8,
    Platinum = 9,
    HeartgoldSoulsilver = 10,
    BlackWhite = 11,
    Colosseum = 12,
    Xd = 13,
    Black2White2 = 14,
    Xy = 15,
    OmegaRubyAlphaSapphire = 16,
    SunMoon = 17,
    UltraSunUltraMoon = 18,
    LetsGoPikachuLetsGoEevee = 19,
    SwordShield = 20,
    TheIsleOfArmor = 21,
    TheCrownTundra = 22,
    BrilliantDiamondShiningPearl = 23,
    LegendsArceus = 24,
    ScarletViolet = 25,
    TheTealMask = 26,
    TheIndigoDisk = 27,
    RedGreenJapan = 28,
    BlueJapan = 29,
    LegendsZA = 30,
    MegaDimension = 31,
}

impl VersionGroup {
    const CHRONOLOGICAL_ORDER: &'static [VersionGroup] = &[
        VersionGroup::RedGreenJapan,
        VersionGroup::BlueJapan,
        VersionGroup::RedBlue,
        VersionGroup::Yellow,
        VersionGroup::GoldSilver,
        VersionGroup::Crystal,
        VersionGroup::RubySapphire,
        VersionGroup::Colosseum,
        VersionGroup::FireredLeafgreen,
        VersionGroup::Emerald,
        VersionGroup::Xd,
        VersionGroup::DiamondPearl,
        VersionGroup::Platinum,
        VersionGroup::HeartgoldSoulsilver,
        VersionGroup::BlackWhite,
        VersionGroup::Black2White2,
        VersionGroup::Xy,
        VersionGroup::OmegaRubyAlphaSapphire,
        VersionGroup::SunMoon,
        VersionGroup::UltraSunUltraMoon,
        VersionGroup::LetsGoPikachuLetsGoEevee,
        VersionGroup::SwordShield,
        VersionGroup::TheIsleOfArmor,
        VersionGroup::TheCrownTundra,
        VersionGroup::BrilliantDiamondShiningPearl,
        VersionGroup::LegendsArceus,
        VersionGroup::ScarletViolet,
        VersionGroup::TheTealMask,
        VersionGroup::TheIndigoDisk,
        VersionGroup::LegendsZA,
        VersionGroup::MegaDimension,
    ];

    fn chronological_index(&self) -> usize {
        Self::CHRONOLOGICAL_ORDER
            .iter()
            .position(|v| v == self)
            .unwrap()
    }

    pub fn prev(&self) -> Option<Self> {
        let idx = self.chronological_index();
        idx.checked_sub(1).map(|i| Self::CHRONOLOGICAL_ORDER[i])
    }

    pub fn next(&self) -> Option<Self> {
        let idx = self.chronological_index();
        Self::CHRONOLOGICAL_ORDER.get(idx + 1).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_chronological_order() {
        for vg in VersionGroup::iter() {
            let _chronological_index = vg.chronological_index();
        }
    }
}
