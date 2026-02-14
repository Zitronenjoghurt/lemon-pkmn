use crate::battle::pokemon::BattlePokemon;
use crate::battle::slot::BattleSlot;
use crate::data::Data;
use crate::error::PkmnResult;
use crate::storage::team::StoredTeam;

pub struct BattleTeam {
    active: Vec<Option<usize>>,
    pokemon: [BattleSlot; 6],
}

impl BattleTeam {
    pub fn new(data: &Data, team: &StoredTeam, slots_per_participant: usize) -> PkmnResult<Self> {
        let mut pokemon: [BattleSlot; 6] = Default::default();
        let mut active: Vec<Option<usize>> = vec![None; slots_per_participant];
        let mut active_idx = 0;

        for (i, slot) in team.all().iter().enumerate() {
            if let Some(stored) = slot {
                pokemon[i] = BattleSlot::Filled(BattlePokemon::new(data, stored)?);
                if active_idx < slots_per_participant {
                    active[active_idx] = Some(i);
                    active_idx += 1;
                }
            }
        }

        Ok(Self { pokemon, active })
    }
}
