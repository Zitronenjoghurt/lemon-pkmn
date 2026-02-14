use crate::battle::target::BattleTargetSingle;
use crate::data::move_id::MoveId;

pub enum BattleAction {
    UseMove {
        source: BattleTargetSingle,
        move_id: MoveId,
    },
}
