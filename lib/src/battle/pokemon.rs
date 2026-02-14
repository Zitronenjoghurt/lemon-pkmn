use crate::storage::pokemon::StoredPokemon;
use crate::types::stat_stages::StatStages;
use crate::types::stats::Stats;

pub struct BattlePokemon {
    base: StoredPokemon,
    stats: Stats<u16>,
    stat_stages: StatStages,
}
