use crate::parser::csv::CsvData;

mod pokemon_move;
mod species;

#[derive(Debug)]
pub struct ParsedData {
    pub moves: Vec<pokemon_move::PokemonMoveRecord>,
    pub species: Vec<species::SpeciesRecord>,
}

impl ParsedData {
    pub fn parse(csv_data: &CsvData) -> anyhow::Result<Self> {
        Ok(Self {
            moves: pokemon_move::PokemonMoveRecord::parse(csv_data)?,
            species: species::SpeciesRecord::parse(csv_data)?,
        })
    }
}

pub trait DataRecord: Sized {
    fn parse(csv_data: &CsvData) -> anyhow::Result<Vec<Self>>;
}
