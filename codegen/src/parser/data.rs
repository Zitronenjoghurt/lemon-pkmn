use crate::parser::csv::CsvData;

mod species;

#[derive(Debug)]
pub struct ParsedData {
    pub species: Vec<species::SpeciesRecord>,
}

impl ParsedData {
    pub fn parse(csv_data: &CsvData) -> anyhow::Result<Self> {
        Ok(Self {
            species: species::SpeciesRecord::parse(csv_data)?,
        })
    }
}

pub trait DataRecord: Sized {
    fn parse(csv_data: &CsvData) -> anyhow::Result<Vec<Self>>;
}
