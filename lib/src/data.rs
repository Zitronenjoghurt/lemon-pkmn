use crate::types::move_damage_class::MoveDamageClass;
use crate::types::move_target::MoveTarget;
use crate::types::pokemon_type::PokemonType;
use crate::types::stats::Stats;

pub mod moves;
pub mod species;

pub struct SpeciesData {
    pub identifier: &'static str,
    pub national_dex: u16,
    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,
    pub base_stats: Stats,
    pub is_default_form: bool,
    pub is_battle_only: bool,
    pub is_mega: bool,
    pub is_gmax: bool,
    pub form_switchable: bool,
    pub form_identifier: Option<&'static str>,
    pub is_legendary: bool,
    pub is_mythical: bool,
}

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
