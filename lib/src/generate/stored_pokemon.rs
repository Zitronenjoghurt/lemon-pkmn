use crate::data::species_id::SpeciesId;
use crate::data::Data;
use crate::error::PkmnResult;
use crate::generate::evs::EvsGenerator;
use crate::generate::ivs::IvsGenerator;
use crate::generate::level::LevelGenerator;
use crate::generate::nature::NatureGenerator;
use crate::generate::species_id::SpeciesIdGenerator;
use crate::generate::stored_moves::MovesGenerator;
use crate::generate::version_group::VersionGroupGenerator;
use crate::storage::pokemon::StoredPokemon;
use crate::types::version_group::VersionGroup;

#[derive(Default)]
pub struct StoredPokemonGenerator {
    species_id: SpeciesIdGenerator,
    moves: MovesGenerator,
    evs: EvsGenerator,
    ivs: IvsGenerator,
    nature: NatureGenerator,
    level: LevelGenerator,
    version_group: VersionGroupGenerator,
}

// ToDo: For evolved pokemon only generate levels above the level they evolve at
impl StoredPokemonGenerator {
    pub fn generate<R: rand::Rng>(&self, rng: &mut R, data: &Data) -> PkmnResult<StoredPokemon> {
        let species_id = self.species_id.generate(rng);
        let species = data.get_species(species_id)?;
        let vg = self.version_group.generate(rng);
        let vg = if vg.chronological_index() < species.first_appearance.chronological_index() {
            species.first_appearance
        } else {
            vg
        };

        let level = self.level.generate(rng);
        let moves = self.moves.generate(data, vg, species_id, level)?;
        let evs = self.evs.generate(rng);
        let ivs = self.ivs.generate(rng);
        let nature = self.nature.generate(rng);

        Ok(StoredPokemon {
            species_id,
            moves,
            evs,
            ivs,
            nature,
            level,
            version_group: vg,
        })
    }

    pub fn species_id(mut self, f: impl FnOnce(SpeciesIdGenerator) -> SpeciesIdGenerator) -> Self {
        self.species_id = f(self.species_id);
        self
    }

    pub fn with_species_id(self, species_id: SpeciesId) -> Self {
        self.species_id(|g| g.specific(species_id))
    }

    pub fn moves(mut self, f: impl FnOnce(MovesGenerator) -> MovesGenerator) -> Self {
        self.moves = f(self.moves);
        self
    }

    pub fn evs(mut self, f: impl FnOnce(EvsGenerator) -> EvsGenerator) -> Self {
        self.evs = f(self.evs);
        self
    }

    pub fn ivs(mut self, f: impl FnOnce(IvsGenerator) -> IvsGenerator) -> Self {
        self.ivs = f(self.ivs);
        self
    }

    pub fn nature(mut self, f: impl FnOnce(NatureGenerator) -> NatureGenerator) -> Self {
        self.nature = f(self.nature);
        self
    }

    pub fn level(mut self, f: impl FnOnce(LevelGenerator) -> LevelGenerator) -> Self {
        self.level = f(self.level);
        self
    }

    pub fn with_level(self, level: u8) -> Self {
        self.level(|g| g.specific(level))
    }

    pub fn with_level_bounds(self, min: u8, max: u8) -> Self {
        self.level(|g| g.bounds(min, max))
    }

    pub fn version_group(
        mut self,
        f: impl FnOnce(VersionGroupGenerator) -> VersionGroupGenerator,
    ) -> Self {
        self.version_group = f(self.version_group);
        self
    }

    pub fn with_version_group(self, vg: VersionGroup) -> Self {
        self.version_group(|g| g.specific(vg))
    }
}
