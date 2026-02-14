use crate::parser::csv::CSVRecord;

#[derive(Debug, serde::Deserialize)]
pub struct PokemonMovesRecord {
    pub pokemon_id: u16,
    pub version_group_id: u8,
    pub move_id: u16,
    pub pokemon_move_method_id: u8,
    pub level: u8,
    #[serde(default)]
    pub order: Option<u8>,
}

impl CSVRecord for PokemonMovesRecord {
    const FILENAME: &'static str = "pokemon_moves";
}
