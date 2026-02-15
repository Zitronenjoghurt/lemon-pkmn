use crate::battle::types::target::BattleTargetSingle;
use crate::data::move_id::MoveId;

#[derive(Debug)]
pub enum BattleEvent {
    DebugMessage(String),
    InfoMessage(String),
    MoveAnnounced {
        source: BattleTargetSingle,
        move_id: MoveId,
    },
    MoveMissed {
        source: BattleTargetSingle,
    },
    Damage {
        target: BattleTargetSingle,
        damage: u16,
    },
    Fainted {
        target: BattleTargetSingle,
    },
}

impl BattleEvent {
    pub fn debug(msg: impl Into<String>) -> Self {
        Self::DebugMessage(msg.into())
    }

    pub fn info(msg: impl Into<String>) -> Self {
        Self::InfoMessage(msg.into())
    }
}
