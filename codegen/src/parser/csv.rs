use anyhow::Context;
use std::path::Path;

mod moves;
pub mod pokemon;
mod pokemon_form_types;
pub mod pokemon_forms;
mod pokemon_species;
mod pokemon_stats;
mod pokemon_types;

pub struct CsvData {
    pub moves: Vec<moves::MovesRecord>,
    pub pokemon: Vec<pokemon::PokemonRecord>,
    pub pokemon_form_types: Vec<pokemon_form_types::PokemonFormTypesRecord>,
    pub pokemon_forms: Vec<pokemon_forms::PokemonFormsRecord>,
    pub pokemon_types: Vec<pokemon_types::PokemonTypesRecord>,
    pub pokemon_species: Vec<pokemon_species::PokemonSpeciesRecord>,
    pub pokemon_stats: Vec<pokemon_stats::PokemonStatsRecord>,
}

impl CsvData {
    pub fn load(data_dir: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            moves: moves::MovesRecord::load(data_dir)?,
            pokemon: pokemon::PokemonRecord::load(data_dir)?,
            pokemon_form_types: pokemon_form_types::PokemonFormTypesRecord::load(data_dir)?,
            pokemon_forms: pokemon_forms::PokemonFormsRecord::load(data_dir)?,
            pokemon_types: pokemon_types::PokemonTypesRecord::load(data_dir)?,
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
