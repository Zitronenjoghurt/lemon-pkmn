use crate::data::move_id::MoveId;

#[derive(Debug, Copy, Clone)]
pub struct StoredMove {
    pub move_id: MoveId,
    pub pp: u8,
}
