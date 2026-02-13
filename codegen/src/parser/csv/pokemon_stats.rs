use crate::parser::csv::CSVRecord;
use anyhow::Context;
use lemon_pkmn::types::stats::Stats;
use strum_macros::FromRepr;

/// Source: https://github.com/PokeAPI/pokeapi/blob/master/data/v2/csv/stats.csv
#[derive(Debug, Copy, Clone, FromRepr)]
#[repr(u8)]
enum PokemonStatsId {
    Hp = 1,
    Atk = 2,
    Def = 3,
    SpAtk = 4,
    SpDef = 5,
    Speed = 6,
    Accuracy = 7,
    Evasion = 8,
    Special = 9,
}

#[derive(Debug, serde::Deserialize)]
pub struct PokemonStatsRecord {
    pub pokemon_id: u16,
    pub stat_id: u8,
    pub base_stat: u8,
    // ToDo: Handle effort
    pub effort: u8,
}

impl CSVRecord for PokemonStatsRecord {
    const FILENAME: &'static str = "pokemon_stats";
}

impl PokemonStatsRecord {
    pub fn apply(&self, stats: &mut Stats) -> anyhow::Result<()> {
        let stat_id = PokemonStatsId::from_repr(self.stat_id)
            .context(format!("Invalid stat id: '{}'", self.stat_id))?;

        match stat_id {
            PokemonStatsId::Hp => stats.hp = self.base_stat,
            PokemonStatsId::Atk => stats.atk = self.base_stat,
            PokemonStatsId::Def => stats.def = self.base_stat,
            PokemonStatsId::SpAtk => stats.sp_atk = self.base_stat,
            PokemonStatsId::SpDef => stats.sp_def = self.base_stat,
            PokemonStatsId::Speed => stats.speed = self.base_stat,
            _ => {
                anyhow::bail!(
                    "Stat with id '{}' cannot be applied to pokemon stats",
                    self.stat_id
                );
            }
        }

        Ok(())
    }
}
