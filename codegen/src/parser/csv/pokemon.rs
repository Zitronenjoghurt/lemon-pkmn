use crate::parser::csv::CSVRecord;

#[derive(Debug, serde::Deserialize)]
pub struct PokemonRecord {
    pub id: u16,
    pub species_id: u16,
    pub is_default: u8,
}

impl CSVRecord for PokemonRecord {
    const FILENAME: &'static str = "pokemon";
}
