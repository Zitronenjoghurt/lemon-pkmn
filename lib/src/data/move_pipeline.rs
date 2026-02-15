use crate::battle::sim::move_executor::MoveStep;
use crate::data::move_id::MoveId;

static PIPELINE_NOT_IMPLEMENTED: &[MoveStep] = &[MoveStep::NotImplemented];

static PIPELINE_DAMAGING: &[MoveStep] = &[
    MoveStep::Announce,
    MoveStep::AccuracyCheck,
    MoveStep::DealDamage,
];

pub fn move_pipeline(move_id: MoveId) -> &'static [MoveStep] {
    match move_id {
        MoveId::Pound => PIPELINE_DAMAGING,
        _ => PIPELINE_NOT_IMPLEMENTED,
    }
}
