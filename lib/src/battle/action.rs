use crate::battle::target::BattleTargetSingle;

#[derive(Debug, Clone)]
pub enum BattleActionKind {
    UseMove {
        move_index: usize,
        target: BattleTargetSingle,
    },
}

#[derive(Debug, Clone)]
pub struct BattleAction {
    pub source: BattleTargetSingle,
    pub kind: BattleActionKind,
}

impl BattleAction {
    pub fn use_move(
        source: BattleTargetSingle,
        target: BattleTargetSingle,
        move_index: usize,
    ) -> Self {
        Self {
            source,
            kind: BattleActionKind::UseMove { move_index, target },
        }
    }
}
