use anyhow::Context;
use std::path::Path;

pub mod pokemon;
pub mod pokemon_forms;
mod pokemon_species;
mod pokemon_stats;

pub struct CsvData {
    pub pokemon: Vec<pokemon::PokemonRecord>,
    pub pokemon_forms: Vec<pokemon_forms::PokemonFormsRecord>,
    pub pokemon_species: Vec<pokemon_species::PokemonSpeciesRecord>,
    pub pokemon_stats: Vec<pokemon_stats::PokemonStatsRecord>,
}

impl CsvData {
    pub fn load(data_dir: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            pokemon: pokemon::PokemonRecord::load(data_dir)?,
            pokemon_forms: pokemon_forms::PokemonFormsRecord::load(data_dir)?,
            pokemon_species: pokemon_species::PokemonSpeciesRecord::load(data_dir)?,
            pokemon_stats: pokemon_stats::PokemonStatsRecord::load(data_dir)?,
        })
    }
}

pub trait CSVRecord: serde::de::DeserializeOwned {
    const FILENAME: &'static str;

    fn load(data_dir: &Path) -> anyhow::Result<Vec<Self>> {
        let path = data_dir.join(format!("{}.csv", Self::FILENAME));
        let mut reader = csv::Reader::from_path(&path)
            .with_context(|| format!("failed to open {}", path.display()))?;
        reader
            .deserialize()
            .collect::<Result<Vec<Self>, _>>()
            .with_context(|| format!("failed to parse {}", path.display()))
    }
}
