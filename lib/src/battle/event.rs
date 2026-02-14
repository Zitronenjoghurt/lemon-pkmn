use crate::battle::target::{BattleTarget, BattleTargetSingle};
use crate::data::move_id::MoveId;

pub enum BattleEvent {
    MoveUsed {
        source: BattleTargetSingle,
        target: BattleTarget,
        move_id: MoveId,
    },
}
