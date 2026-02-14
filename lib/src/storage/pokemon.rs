use crate::data::species_id::SpeciesId;
use crate::storage::pokemon_move::StoredMove;
use crate::types::nature::Nature;
use crate::types::stats::Stats;

#[derive(Debug)]
pub struct StoredPokemon {
    pub species_id: SpeciesId,
    pub moves: [Option<StoredMove>; 4],
    /// Effort values, earned through battling (see species ev yield)
    /// Limited to 0-252 per stat and 510 across all stats
    pub evs: Stats<u8>,
    /// Individual values, innate stat bonuses
    /// Ranging between 0-31
    pub ivs: Stats<u8>,
    pub nature: Nature,
    pub level: u8,
}
