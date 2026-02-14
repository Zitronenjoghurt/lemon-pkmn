use crate::types::stats::Stats;

#[derive(Default)]
pub struct EvsGenerator {
    hp: Option<u8>,
    atk: Option<u8>,
    def: Option<u8>,
    sp_atk: Option<u8>,
    sp_def: Option<u8>,
    speed: Option<u8>,
}

impl EvsGenerator {
    pub fn generate<R: rand::Rng>(&self, _rng: &mut R) -> Stats<u8> {
        let hp = if let Some(hp) = self.hp { hp } else { 0 };

        let atk = if let Some(atk) = self.atk { atk } else { 0 };

        let def = if let Some(def) = self.def { def } else { 0 };

        let sp_atk = if let Some(sp_atk) = self.sp_atk {
            sp_atk
        } else {
            0
        };

        let sp_def = if let Some(sp_def) = self.sp_def {
            sp_def
        } else {
            0
        };

        let speed = if let Some(speed) = self.speed {
            speed
        } else {
            0
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
