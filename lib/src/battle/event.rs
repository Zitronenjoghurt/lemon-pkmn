use crate::battle::target::BattleTargetSingle;
use crate::data::move_id::MoveId;

#[derive(Debug)]
pub enum BattleEvent {
    MoveAnnounced {
        source: BattleTargetSingle,
        move_id: MoveId,
    },
    Damage {
        target: BattleTargetSingle,
        damage: u16,
    },
    Fainted {
        target: BattleTargetSingle,
    },
}
