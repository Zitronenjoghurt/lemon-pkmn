use crate::parser::csv::CSVRecord;

#[derive(Debug, serde::Deserialize)]
pub struct PokemonTypesRecord {
    pub pokemon_id: u16,
    pub type_id: u16,
    pub slot: u8,
}

impl CSVRecord for PokemonTypesRecord {
    const FILENAME: &'static str = "pokemon_types";
}
