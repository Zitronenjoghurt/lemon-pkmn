use crate::types::stats::Stats;
use rand::RngExt;

#[derive(Default)]
pub struct IvsGenerator {
    hp: Option<u8>,
    atk: Option<u8>,
    def: Option<u8>,
    sp_atk: Option<u8>,
    sp_def: Option<u8>,
    speed: Option<u8>,
}

impl IvsGenerator {
    pub fn generate<R: rand::Rng>(&self, rng: &mut R) -> Stats<u8> {
        let hp = if let Some(hp) = self.hp {
            hp
        } else {
            rng.random_range(0..=31)
        };

        let atk = if let Some(atk) = self.atk {
            atk
        } else {
            rng.random_range(0..=31)
        };

        let def = if let Some(def) = self.def {
            def
        } else {
            rng.random_range(0..=31)
        };

        let sp_atk = if let Some(sp_atk) = self.sp_atk {
            sp_atk
        } else {
            rng.random_range(0..=31)
        };

        let sp_def = if let Some(sp_def) = self.sp_def {
            sp_def
        } else {
            rng.random_range(0..=31)
        };

        let speed = if let Some(speed) = self.speed {
            speed
        } else {
            rng.random_range(0..=31)
        };

        Stats {
            hp,
            atk,
            def,
            sp_atk,
            sp_def,
            speed,
        }
    }
}
