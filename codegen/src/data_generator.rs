use crate::parser::data::ParsedData;
use lemon_pkmn::data::{Data, MoveData, SpeciesData};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct DataGenerator {
    output_file: PathBuf,
}

impl DataGenerator {
    pub fn new(output_file: &Path) -> Self {
        Self {
            output_file: output_file.to_owned(),
        }
    }

    pub fn generate(&self, data: ParsedData) -> anyhow::Result<()> {
        let data = Data {
            species: self.generate_species(&data),
            moves: self.generate_moves(&data),
        };

        let data = bitcode::encode(&data);
        let compressed = zstd::encode_all(data.as_slice(), 22)?;
        std::fs::write(self.output_file.clone(), compressed)?;

        Ok(())
    }

    fn generate_species(&self, data: &ParsedData) -> HashMap<u16, SpeciesData> {
        data.species
            .iter()
            .map(|species| {
                (
                    species.id,
                    SpeciesData {
                        identifier: species.identifier.clone(),
                        national_dex: species.national_id,
                        primary_type: species.primary_type,
                        secondary_type: species.secondary_type,
                        base_stats: species.stats,
                        ev_yield: species.ev_yield,
                        form_identifier: species.form_identifier.clone(),
                        flags: species.flags,
                        moveset: species.moveset.clone(),
                    },
                )
            })
            .collect()
    }

    fn generate_moves(&self, data: &ParsedData) -> HashMap<u16, MoveData> {
        data.moves
            .iter()
            .map(|m| {
                (
                    m.id,
                    MoveData {
                        identifier: m.identifier.clone(),
                        pokemon_type: m.pokemon_type,
                        power: m.power,
                        pp: m.pp,
                        accuracy: m.accuracy,
                        priority: m.priority,
                        target: m.target,
                        damage_class: m.damage_class,
                    },
                )
            })
            .collect()
    }
}
