use crate::types::stats::Stats;

pub mod species;

pub struct SpeciesData {
    pub identifier: &'static str,
    pub national_dex: u16,
    pub base_stats: Stats,
    pub is_default_form: bool,
    pub is_battle_only: bool,
    pub is_mega: bool,
    pub is_gmax: bool,
}
