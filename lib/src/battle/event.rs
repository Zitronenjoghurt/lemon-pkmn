use crate::battle::target::{BattleTarget, BattleTargetSingle};
use crate::types::pokemon_move::MoveId;

pub enum BattleEvent {
    MoveUsed {
        source: BattleTargetSingle,
        target: BattleTarget,
        move_id: MoveId,
    },
}
