use crate::types::nature::Nature;
use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct NatureGenerator {
    specific: Option<Nature>,
}

impl NatureGenerator {
    pub fn generate<R: rand::Rng>(&self, rng: &mut R) -> Nature {
        if let Some(nature) = self.specific {
            nature
        } else {
            Nature::iter().choose(rng).unwrap()
        }
    }
}
