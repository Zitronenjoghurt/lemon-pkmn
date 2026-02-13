use crate::parser::csv::CSVRecord;

#[derive(Debug, serde::Deserialize)]
pub struct PokemonFormsRecord {
    pub id: u16,
    pub identifier: String,
    pub form_identifier: String,
    pub pokemon_id: u16,
    pub is_default: u8,
    pub is_battle_only: u8,
    pub is_mega: u8,
}

impl CSVRecord for PokemonFormsRecord {
    const FILENAME: &'static str = "pokemon_forms";
}
