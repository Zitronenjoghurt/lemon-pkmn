use crate::battle::target::BattleTargetSingle;
use crate::data::moves::MoveId;

pub enum BattleAction {
    UseMove {
        source: BattleTargetSingle,
        move_id: MoveId,
    },
}
