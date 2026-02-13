use crate::parser::csv::CSVRecord;

#[derive(Debug, serde::Deserialize)]
pub struct PokemonFormTypesRecord {
    pub pokemon_form_id: u16,
    pub type_id: u16,
    pub slot: u8,
}

impl CSVRecord for PokemonFormTypesRecord {
    const FILENAME: &'static str = "pokemon_form_types";
}
