use crate::data::moves::MoveId;
use crate::data::species::SpeciesId;
use crate::error::{PkmnError, PkmnResult};
use crate::types::move_damage_class::MoveDamageClass;
use crate::types::move_target::MoveTarget;
use crate::types::pokemon_type::PokemonType;
use crate::types::species_flags::SpeciesFlags;
use crate::types::stats::Stats;
use std::collections::HashMap;

pub mod moves;
pub mod species;

#[cfg(feature = "include-data")]
const RAW_DATA: &[u8] = include_bytes!("../../data.bin");

#[derive(Debug)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Data {
    pub species: HashMap<u16, SpeciesData>,
    pub moves: HashMap<u16, MoveData>,
}

impl Data {
    #[cfg(feature = "include-data")]
    pub fn load_included() -> PkmnResult<Self> {
        let decompressed = zstd::decode_all(RAW_DATA)?;
        Ok(bitcode::decode(&decompressed)?)
    }

    pub fn get_species(&self, species_id: SpeciesId) -> PkmnResult<&SpeciesData> {
        self.species
            .get(&(species_id as u16))
            .ok_or(PkmnError::SpeciesNotFound(species_id))
    }

    pub fn get_move(&self, move_id: MoveId) -> PkmnResult<&MoveData> {
        self.moves
            .get(&(move_id as u16))
            .ok_or(PkmnError::MoveNotFound(move_id))
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpeciesData {
    pub identifier: String,
    pub national_dex: u16,
    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,
    pub base_stats: Stats<u8>,
    pub ev_yield: Stats<u8>,
    pub form_identifier: Option<String>,
    pub flags: SpeciesFlags,
}

#[derive(Debug)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MoveData {
    pub identifier: String,
    pub pokemon_type: PokemonType,
    pub power: u8,
    pub pp: u8,
    pub accuracy: u8,
    pub priority: i8,
    pub target: MoveTarget,
    pub damage_class: MoveDamageClass,
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "include-data")]
    fn test_included_data() {
        use super::*;
        use strum::IntoEnumIterator;

        let data = Data::load_included().unwrap();

        for species_id in SpeciesId::iter() {
            data.get_species(species_id).unwrap();
        }

        for move_id in MoveId::iter() {
            data.get_move(move_id).unwrap();
        }
    }
}
