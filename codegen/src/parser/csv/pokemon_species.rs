use crate::parser::csv::CSVRecord;

#[derive(Debug, serde::Deserialize)]
pub struct PokemonSpeciesRecord {
    pub id: u16,
    pub forms_switchable: u8,
    pub is_legendary: u8,
    pub is_mythical: u8,
}

impl CSVRecord for PokemonSpeciesRecord {
    const FILENAME: &'static str = "pokemon_species";
}
