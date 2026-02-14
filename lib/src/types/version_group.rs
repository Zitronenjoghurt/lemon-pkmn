use strum_macros::{Display, EnumIter, FromRepr};

// ToDo: Auto-generate
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, FromRepr)]
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
