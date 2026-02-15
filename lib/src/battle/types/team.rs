use crate::battle::config::BattleConfig;
use crate::battle::types::pokemon::BattlePokemon;
use crate::battle::types::slot::BattleSlot;
use crate::data::Data;
use crate::error::PkmnResult;
use crate::storage::team::StoredTeam;

pub struct BattleTeam {
    active: Vec<Option<usize>>,
    pokemon: [BattleSlot; 6],
}

impl BattleTeam {
    pub fn new(data: &Data, team: &StoredTeam, config: &BattleConfig) -> PkmnResult<Self> {
        let mut pokemon: [BattleSlot; 6] = Default::default();
        let mut active: Vec<Option<usize>> = vec![None; config.active_slots_per_team];
        let mut active_idx = 0;

        for (i, slot) in team.all().iter().enumerate() {
            if let Some(stored) = slot {
                pokemon[i] = BattleSlot::Filled(BattlePokemon::new(data, stored)?);
                if active_idx < config.active_slots_per_team {
                    active[active_idx] = Some(i);
                    active_idx += 1;
                }
            }
        }

        Ok(Self { pokemon, active })
    }

    fn pokemon_index_from_active_slot_index(&self, slot_index: usize) -> Option<usize> {
        *self.active.get(slot_index)?
    }

    pub fn get_active_pokemon(&self, slot_index: usize) -> Option<&BattlePokemon> {
        let pokemon_index = self.pokemon_index_from_active_slot_index(slot_index)?;
        let slot = self.pokemon.get(pokemon_index)?;
        match slot {
            BattleSlot::Filled(pokemon) => Some(pokemon),
            _ => None,
        }
    }

    pub fn get_active_pokemon_mut(&mut self, slot_index: usize) -> Option<&mut BattlePokemon> {
        let pokemon_index = self.pokemon_index_from_active_slot_index(slot_index)?;
        let slot = self.pokemon.get_mut(pokemon_index)?;
        match slot {
            BattleSlot::Filled(pokemon) => Some(pokemon),
            _ => None,
        }
    }
}
