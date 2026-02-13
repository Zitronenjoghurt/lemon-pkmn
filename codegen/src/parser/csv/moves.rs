use crate::parser::csv::CSVRecord;

#[derive(Debug, serde::Deserialize)]
pub struct MovesRecord {
    pub id: u16,
    pub identifier: String,
    pub type_id: u16,
    #[serde(default)]
    pub power: Option<u8>,
    #[serde(default)]
    pub pp: Option<u8>,
    #[serde(default)]
    pub accuracy: Option<u8>,
    pub priority: i8,
    pub target_id: u8,
    pub damage_class_id: u8,
}

impl CSVRecord for MovesRecord {
    const FILENAME: &'static str = "moves";
}
