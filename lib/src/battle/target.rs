use crate::battle::side::BattleSide;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BattleTarget {
    Single(BattleTargetSingle),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BattleTargetSingle {
    side: BattleSide,
    slot: usize,
}
