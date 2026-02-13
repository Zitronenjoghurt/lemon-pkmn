#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BattleSide {
    A,
    B
}

impl BattleSide {
    pub fn other(&self) -> BattleSide {
        match self {
            BattleSide::A => BattleSide::B,
            BattleSide::B => BattleSide::A,
        }
    }
}