use crate::data::move_id::MoveId;
use crate::error::{PkmnError, PkmnResult};
use crate::types::move_method::MoveMethod;
use crate::types::version_group::VersionGroup;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Moveset(HashMap<VersionGroup, MovesetEntries>);

impl Moveset {
    pub fn get(&self, version_group: VersionGroup) -> Option<&MovesetEntries> {
        self.0.get(&version_group)
    }

    pub fn insert(&mut self, version_group: VersionGroup, entry: MovesetEntry) {
        self.0
            .entry(version_group)
            .or_insert(MovesetEntries(vec![]))
            .push(entry);
    }

    /// Returns up to 4 of the most recent (version-wise) learned (level-wise) moves.\
    /// Will fall back to the most recent earlier version if there are no moves for the specified one.
    pub fn get_moves_by_level(&self, version_group: VersionGroup, level: u8) -> Vec<MoveId> {
        let mut vg = version_group;
        let entries = loop {
            if let Some(entries) = self.get(vg) {
                break entries;
            }
            match vg.prev() {
                Some(prev) => vg = prev,
                None => return vec![],
            }
        };

        let mut level_up_moves: Vec<&MovesetEntry> = entries
            .entries()
            .iter()
            .filter(|e| matches!(e.method, MoveMethod::LevelUp) && e.level <= level)
            .collect();
        level_up_moves.sort_by(|a, b| a.level.cmp(&b.level).then_with(|| a.order.cmp(&b.order)));
        level_up_moves.dedup_by_key(|e| e.move_id);

        level_up_moves
            .iter()
            .rev()
            .take(4)
            .rev()
            .filter_map(|e| e.move_id().ok())
            .collect()
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MovesetEntries(Vec<MovesetEntry>);

impl MovesetEntries {
    pub fn entries(&self) -> &[MovesetEntry] {
        &self.0
    }

    pub fn push(&mut self, entry: MovesetEntry) {
        self.0.push(entry);
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MovesetEntry {
    pub move_id: u16,
    pub level: u8,
    pub order: Option<u8>,
    pub method: MoveMethod,
}

impl MovesetEntry {
    pub fn move_id(&self) -> PkmnResult<MoveId> {
        MoveId::from_repr(self.move_id).ok_or(PkmnError::MoveNotFound(self.move_id))
    }
}
