use rand::RngExt;

#[derive(Default)]
pub struct LevelGenerator {
    specific: Option<u8>,
    min: Option<u8>,
    max: Option<u8>,
}

impl LevelGenerator {
    pub fn generate<R: rand::Rng>(&self, rng: &mut R) -> u8 {
        if let Some(level) = self.specific {
            level
        } else {
            let min = self.min.unwrap_or(1).clamp(1, 100);
            let max = self.max.unwrap_or(100).clamp(min, 100);

            if min <= max {
                rng.random_range(min..=max)
            } else {
                rng.random_range(max..=min)
            }
        }
    }

    pub fn specific(mut self, level: u8) -> Self {
        self.specific = Some(level);
        self
    }

    pub fn min(mut self, min: u8) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: u8) -> Self {
        self.max = Some(max);
        self
    }

    pub fn bounds(mut self, min: u8, max: u8) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }
}
