use crate::data::move_id::MoveId;
use crate::data::species_id::SpeciesId;
use crate::data::Data;
use crate::error::PkmnResult;
use crate::storage::pokemon_move::StoredMove;
use crate::types::version_group::VersionGroup;

#[derive(Default)]
pub struct MovesGenerator {
    specific: Vec<MoveId>,
}

impl MovesGenerator {
    pub fn generate(
        &self,
        data: &Data,
        version_group: VersionGroup,
        species_id: SpeciesId,
        level: u8,
    ) -> PkmnResult<[Option<StoredMove>; 4]> {
        let species = data.get_species(species_id)?;
        let mut move_ids = [None; 4];

        let level_up_moves = species.moveset.get_moves_by_level(version_group, level);
        for (i, move_id) in level_up_moves.iter().enumerate() {
            move_ids[i] = Some(*move_id);
        }

        for (i, move_id) in self.specific.iter().enumerate() {
            move_ids[i] = Some(*move_id);
        }

        let mut moves = [None; 4];
        for (i, move_id) in move_ids.iter().enumerate() {
            let Some(move_id) = move_id else {
                continue;
            };
            let move_data = data.get_move(*move_id)?;
            let stored_move = StoredMove {
                move_id: *move_id,
                pp: move_data.pp,
            };
            moves[i] = Some(stored_move);
        }

        Ok(moves)
    }

    pub fn specific(mut self, moves: &[MoveId]) -> Self {
        self.specific = moves.to_vec();
        self
    }
}
