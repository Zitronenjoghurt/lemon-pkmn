use crate::data::Data;
use crate::error::PkmnResult;
use crate::storage::pokemon::StoredPokemon;
use crate::types::stat_stages::StatStages;
use crate::types::stats::Stats;

#[derive(Debug, Clone)]
pub struct BattlePokemon {
    base: StoredPokemon,
    stats: Stats<u16>,
    stat_stages: StatStages,
}

impl BattlePokemon {
    pub fn new(data: &Data, base: &StoredPokemon) -> PkmnResult<Self> {
        let species = data.get_species(base.species_id)?;
        let stats = Stats::compute(
            base.level,
            species.base_stats,
            base.evs,
            base.ivs,
            base.nature,
        );

        Ok(Self {
            base: base.clone(),
            stats,
            stat_stages: StatStages::default(),
        })
    }
}
