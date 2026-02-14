use crate::battle::side::BattleSide;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BattleTarget {
    Single(BattleTargetSingle),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BattleTargetSingle {
    pub side: BattleSide,
    pub slot: usize,
}

impl BattleTargetSingle {
    pub fn new(side: BattleSide, slot: usize) -> Self {
        Self { side, slot }
    }

    pub fn new_a(slot: usize) -> Self {
        Self::new(BattleSide::A, slot)
    }

    pub fn new_b(slot: usize) -> Self {
        Self::new(BattleSide::B, slot)
    }
}

impl Display for BattleTargetSingle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Side {} Slot {} (from left to right)",
            self.side,
            self.slot + 1
        )
    }
}
