use crate::data::species::SpeciesId;
use crate::storage::pokemon_move::StoredMove;
use crate::types::nature::Nature;
use crate::types::stats::Stats;

pub struct StoredPokemon {
    species_id: SpeciesId,
    moves: [Option<StoredMove>; 4],
    /// Effort values, earned through battling (see species ev yield)
    /// Limited to 0-252 per stat and 510 across all stats
    evs: Stats<u8>,
    /// Individual values, innate stat bonuses
    /// Ranging between 0-31
    ivs: Stats<u8>,
    nature: Nature,
    level: u8,
}
