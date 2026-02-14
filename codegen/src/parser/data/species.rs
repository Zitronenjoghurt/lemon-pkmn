use crate::parser::csv::pokemon_forms::PokemonFormsRecord;
use crate::parser::csv::CsvData;
use crate::parser::data::DataRecord;
use anyhow::Context;
use lemon_pkmn::data::moveset::{Moveset, MovesetEntry};
use lemon_pkmn::types::move_method::MoveMethod;
use lemon_pkmn::types::pokemon_type::PokemonType;
use lemon_pkmn::types::species_flags::SpeciesFlags;
use lemon_pkmn::types::stats::{Stat, Stats};
use lemon_pkmn::types::version_group::VersionGroup;
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
    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,
    pub form_identifier: Option<String>,
    pub stats: Stats<u8>,
    pub ev_yield: Stats<u8>,
    pub flags: SpeciesFlags,
    pub moveset: Moveset,
}

#[derive(Debug, Default)]
struct SpeciesBuilder {
    pub identifier: String,
    pub id: u16,
    pub pokemon_id: u16,
    pub national_id: Option<u16>,
    pub primary_type: Option<PokemonType>,
    pub secondary_type: Option<PokemonType>,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_gmax: bool,
    pub is_mega: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub form_switchable: bool,
    pub form_identifier: Option<String>,
    pub stats: Stats<u8>,
    pub ev_yield: Stats<u8>,
    pub moveset: Moveset,
}

impl SpeciesBuilder {
    pub fn build(self) -> anyhow::Result<SpeciesRecord> {
        let mut flags = SpeciesFlags::empty();
        if self.is_default {
            flags.insert(SpeciesFlags::DEFAULT_FORM);
        }
        if self.is_battle_only {
            flags.insert(SpeciesFlags::BATTLE_ONLY);
        }
        if self.is_gmax {
            flags.insert(SpeciesFlags::GMAX);
        }
        if self.is_mega {
            flags.insert(SpeciesFlags::MEGA);
        }
        if self.is_legendary {
            flags.insert(SpeciesFlags::LEGENDARY);
        }
        if self.is_mythical {
            flags.insert(SpeciesFlags::MYTHICAL);
        }
        if self.form_switchable {
            flags.insert(SpeciesFlags::FORM_SWITCHABLE);
        }

        Ok(SpeciesRecord {
            identifier: self.identifier,
            id: self.id,
            pokemon_id: self.pokemon_id,
            national_id: self
                .national_id
                .context(format!("Missing species id for form '{}'", self.id))?,
            primary_type: self
                .primary_type
                .context(format!("Missing primary type for form {}", self.id))?,
            secondary_type: self.secondary_type,
            form_identifier: self.form_identifier,
            stats: self.stats,
            ev_yield: self.ev_yield,
            flags,
            moveset: self.moveset,
        })
    }
}

impl From<&PokemonFormsRecord> for SpeciesBuilder {
    fn from(form: &PokemonFormsRecord) -> Self {
        Self {
            identifier: form.identifier.clone(),
            id: form.id,
            pokemon_id: form.pokemon_id,
            is_battle_only: form.is_battle_only == 1,
            is_gmax: form.form_identifier.as_deref() == Some("gmax"),
            is_mega: form.is_mega == 1,
            form_identifier: form.form_identifier.clone(),
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
                species_by_id.get_mut(id).unwrap().is_default = pokemon.is_default == 1;
                ids_by_national_id
                    .entry(pokemon.species_id)
                    .or_default()
                    .push(*id);
                national_id_by_id.insert(*id, pokemon.species_id);
            }
        }

        for species in &csv_data.pokemon_species {
            let ids = ids_by_national_id.get(&species.id).context(format!(
                "PokeAPI species with id '{}' has no form",
                species.id
            ))?;
            for id in ids {
                species_by_id.get_mut(id).unwrap().form_switchable = species.forms_switchable == 1;
                species_by_id.get_mut(id).unwrap().is_legendary = species.is_legendary == 1;
                species_by_id.get_mut(id).unwrap().is_mythical = species.is_mythical == 1;
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
                let stat = Stat::from_repr(stats_record.stat_id)
                    .context(format!("Invalid stat_id: '{}'", stats_record.stat_id))?;
                species.stats.set(stat, stats_record.base_stat);
                species.ev_yield.set(stat, stats_record.effort);
            }
        }

        for pokemon_type in &csv_data.pokemon_types {
            let ids = ids_by_pokemon_id
                .get(&pokemon_type.pokemon_id)
                .context(format!(
                    "PokeAPI pokemon with id '{}' has no form",
                    pokemon_type.pokemon_id
                ))?;

            for id in ids {
                let species = species_by_id.get_mut(id).unwrap();
                let type_ = PokemonType::from_repr(pokemon_type.type_id).context(format!(
                    "Invalid pokemon type_id: '{}'",
                    pokemon_type.type_id
                ))?;
                if pokemon_type.slot == 1 {
                    species.primary_type = Some(type_);
                } else if pokemon_type.slot == 2 {
                    species.secondary_type = Some(type_);
                }
            }
        }

        for pokemon_form_type in &csv_data.pokemon_form_types {
            let species = species_by_id
                .get_mut(&pokemon_form_type.pokemon_form_id)
                .unwrap();
            let type_ = PokemonType::from_repr(pokemon_form_type.type_id).context(format!(
                "Invalid pokemon type_id: '{}'",
                pokemon_form_type.type_id
            ))?;
            if pokemon_form_type.slot == 1 {
                species.primary_type = Some(type_);
            } else if pokemon_form_type.slot == 2 {
                species.secondary_type = Some(type_);
            }
        }

        for pokemon_move in &csv_data.pokemon_moves {
            let ids = ids_by_pokemon_id
                .get(&pokemon_move.pokemon_id)
                .context(format!(
                    "PokeAPI pokemon with id '{}' has no form",
                    pokemon_move.pokemon_id
                ))?;

            let version_group =
                VersionGroup::from_repr(pokemon_move.version_group_id).context(format!(
                    "Invalid version_group_id: '{}'",
                    pokemon_move.version_group_id
                ))?;

            let move_method =
                MoveMethod::from_repr(pokemon_move.pokemon_move_method_id).context(format!(
                    "Invalid pokemon_move_method_id: '{}'",
                    pokemon_move.pokemon_move_method_id
                ))?;

            let entry = MovesetEntry {
                move_id: pokemon_move.move_id,
                level: pokemon_move.level,
                order: pokemon_move.order,
                method: move_method,
            };

            for id in ids {
                let species = species_by_id.get_mut(id).unwrap();
                species.moveset.insert(version_group, entry);
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
