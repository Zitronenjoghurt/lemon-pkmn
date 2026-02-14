use strum_macros::{Display, EnumIter, FromRepr};

// ToDo: Auto-generate
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, FromRepr)]
#[repr(u8)]
pub enum Generation {
    I = 1,
    II = 2,
    III = 3,
    IV = 4,
    V = 5,
    VI = 6,
    VII = 7,
    VIII = 8,
    IX = 9,
}
