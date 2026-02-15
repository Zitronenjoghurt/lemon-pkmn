use crate::data::Data;
use crate::error::PkmnResult;
use crate::storage::pokemon::StoredPokemon;
use crate::storage::pokemon_move::StoredMove;
use crate::types::stat_stages::StatStages;
use crate::types::stats::Stats;

#[derive(Debug, Clone)]
pub struct BattlePokemon {
    pub base: StoredPokemon,
    pub current_hp: u16,
    pub stats: Stats<u16>,
    pub stat_stages: StatStages,
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
            current_hp: stats.hp,
            stats,
            stat_stages: StatStages::default(),
        })
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    pub fn get_move(&self, move_index: usize) -> Option<StoredMove> {
        self.base.moves.get(move_index).copied()?
    }

    pub fn apply_damage(&mut self, damage: u16) -> u16 {
        if damage > self.current_hp {
            let dealt = self.current_hp;
            self.current_hp = 0;
            dealt
        } else {
            self.current_hp -= damage;
            damage
        }
    }
}
