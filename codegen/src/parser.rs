use std::path::PathBuf;

mod csv;
pub mod data;

pub struct Parser {
    data_dir: PathBuf,
}

impl Parser {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub fn parse_all(&self) -> anyhow::Result<data::ParsedData> {
        let csv_data = csv::CsvData::load(&self.data_dir)?;
        data::ParsedData::parse(&csv_data)
    }
}
