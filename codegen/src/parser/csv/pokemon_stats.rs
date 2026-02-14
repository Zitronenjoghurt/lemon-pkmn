use crate::parser::csv::CSVRecord;

#[derive(Debug, serde::Deserialize)]
pub struct PokemonStatsRecord {
    pub pokemon_id: u16,
    pub stat_id: u8,
    pub base_stat: u8,
    pub effort: u8,
}

impl CSVRecord for PokemonStatsRecord {
    const FILENAME: &'static str = "pokemon_stats";
}
