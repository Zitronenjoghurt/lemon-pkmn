use crate::parser::csv::pokemon_forms::PokemonFormsRecord;
use crate::parser::csv::CsvData;
use crate::parser::data::DataRecord;
use anyhow::Context;
use lemon_pkmn::types::stats::Stats;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SpeciesRecord {
    pub identifier: String,
    /// Unique id per form (from pokemon_forms.csv id)
    pub id: u16,
    /// The pokeapi pokemon id of this form
    pub pokemon_id: u16,
    /// National dex number (from pokemon_species.csv id)
    /// Shared across all forms of the same species
    pub national_id: u16,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_gmax: bool,
    pub is_mega: bool,
    pub stats: Stats,
}

#[derive(Debug, Default)]
struct SpeciesBuilder {
    pub identifier: String,
    pub id: u16,
    pub pokemon_id: u16,
    pub national_id: Option<u16>,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_gmax: bool,
    pub is_mega: bool,
    pub stats: Stats,
}

impl SpeciesBuilder {
    pub fn build(self) -> anyhow::Result<SpeciesRecord> {
        Ok(SpeciesRecord {
            identifier: self.identifier,
            id: self.id,
            pokemon_id: self.pokemon_id,
            national_id: self
                .national_id
                .context(format!("Missing species id for form '{}'", self.id))?,
            is_default: self.is_default,
            is_battle_only: self.is_battle_only,
            is_gmax: self.is_gmax,
            is_mega: self.is_mega,
            stats: self.stats,
        })
    }
}

impl From<&PokemonFormsRecord> for SpeciesBuilder {
    fn from(form: &PokemonFormsRecord) -> Self {
        Self {
            identifier: form.identifier.clone(),
            id: form.id,
            pokemon_id: form.pokemon_id,
            is_default: form.is_default == 1,
            is_battle_only: form.is_battle_only == 1,
            is_gmax: form.form_identifier == "gmax",
            is_mega: form.is_mega == 1,
            ..Default::default()
        }
    }
}

impl DataRecord for SpeciesRecord {
    fn parse(csv_data: &CsvData) -> anyhow::Result<Vec<Self>> {
        let mut species_by_id: HashMap<u16, SpeciesBuilder> = HashMap::new();
        let mut ids_by_pokemon_id: HashMap<u16, Vec<u16>> = HashMap::new();
        let mut pokemon_id_by_id: HashMap<u16, u16> = HashMap::new();
        let mut ids_by_national_id: HashMap<u16, Vec<u16>> = HashMap::new();
        let mut national_id_by_id: HashMap<u16, u16> = HashMap::new();

        for form in &csv_data.pokemon_forms {
            let species_builder = SpeciesBuilder::from(form);
            species_by_id.insert(form.id, species_builder);
            ids_by_pokemon_id
                .entry(form.pokemon_id)
                .or_default()
                .push(form.id);
            pokemon_id_by_id.insert(form.id, form.pokemon_id);
        }

        for pokemon in &csv_data.pokemon {
            let ids = ids_by_pokemon_id.get(&pokemon.id).context(format!(
                "PokeAPI pokemon with id '{}' has no form",
                pokemon.id
            ))?;

            for id in ids {
                species_by_id.get_mut(id).unwrap().national_id = Some(pokemon.species_id);
                ids_by_national_id
                    .entry(pokemon.species_id)
                    .or_default()
                    .push(*id);
                national_id_by_id.insert(*id, pokemon.species_id);
            }
        }

        for stats_record in &csv_data.pokemon_stats {
            let ids = ids_by_pokemon_id
                .get(&stats_record.pokemon_id)
                .context(format!(
                    "PokeAPI pokemon with id '{}' has no form",
                    stats_record.pokemon_id
                ))?;

            for id in ids {
                let species = species_by_id.get_mut(id).unwrap();
                stats_record.apply(&mut species.stats)?;
            }
        }

        let mut species = species_by_id
            .into_values()
            .map(|builder| builder.build())
            .collect::<Result<Vec<_>, _>>()?;
        species.sort_by_key(|species| species.id);
        Ok(species)
    }
}
