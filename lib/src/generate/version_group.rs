use crate::types::version_group::VersionGroup;
use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct VersionGroupGenerator {
    specific: Option<VersionGroup>,
}

impl VersionGroupGenerator {
    pub fn generate<R: rand::Rng>(&self, rng: &mut R) -> VersionGroup {
        if let Some(vg) = self.specific {
            vg
        } else {
            VersionGroup::iter().choose(rng).unwrap()
        }
    }

    pub fn specific(mut self, vg: VersionGroup) -> Self {
        self.specific = Some(vg);
        self
    }
}
