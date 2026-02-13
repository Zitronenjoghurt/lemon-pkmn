use crate::battle::target::BattleTargetSingle;
use crate::types::pokemon_move::MoveId;

pub enum BattleAction {
    UseMove {
        source: BattleTargetSingle,
        move_id: MoveId,
    },
}
