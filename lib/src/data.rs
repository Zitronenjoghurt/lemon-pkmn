use crate::data::moves::MoveId;
use crate::data::species::SpeciesId;
use crate::types::move_damage_class::MoveDamageClass;
use crate::types::move_target::MoveTarget;
use crate::types::pokemon_type::PokemonType;
use crate::types::species_flags::SpeciesFlags;
use crate::types::stats::Stats;

pub mod moves;
pub mod species;

#[derive(Debug)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemSize, mem_dbg::MemDbg))]
pub struct SpeciesData {
    pub identifier: &'static str,
    pub national_dex: u16,
    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,
    pub base_stats: Stats<u8>,
    pub ev_yield: Stats<u8>,
    pub form_identifier: Option<&'static str>,
    pub flags: u8,
}

impl SpeciesData {
    pub fn flags(&self) -> SpeciesFlags {
        SpeciesFlags::from_bits_truncate(self.flags)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemSize, mem_dbg::MemDbg))]
pub struct MoveData {
    pub identifier: &'static str,
    pub pokemon_type: PokemonType,
    pub power: u8,
    pub pp: u8,
    pub accuracy: u8,
    pub priority: i8,
    pub target: MoveTarget,
    pub damage_class: MoveDamageClass,
}

#[cfg(feature = "mem_dbg")]
pub fn print_data_size() {
    use mem_dbg::{MemSize, SizeFlags};
    use strum::IntoEnumIterator;

    let species_total: usize = SpeciesId::iter()
        .map(|s| s.data().mem_size(SizeFlags::default()))
        .sum();

    let move_total: usize = MoveId::iter()
        .map(|m| m.data().mem_size(SizeFlags::default()))
        .sum();

    println!(
        "Species data: {:.1}KB ({} entries)",
        species_total as f64 / 1024.0,
        SpeciesId::iter().len()
    );
    println!(
        "Move data:    {:.1}KB ({} entries)",
        move_total as f64 / 1024.0,
        MoveId::iter().len()
    );
    println!(
        "Total:        {:.1}KB",
        (species_total + move_total) as f64 / 1024.0
    );
}
