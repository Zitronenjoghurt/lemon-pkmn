use crate::data::species_id::SpeciesId;
use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct SpeciesIdGenerator {
    specific: Option<SpeciesId>,
}

impl SpeciesIdGenerator {
    pub fn generate<R: rand::Rng>(&self, rng: &mut R) -> SpeciesId {
        match self.specific {
            Some(id) => id,
            None => SpeciesId::iter().choose(rng).unwrap(),
        }
    }

    pub fn specific(mut self, id: SpeciesId) -> Self {
        self.specific = Some(id);
        self
    }
}
