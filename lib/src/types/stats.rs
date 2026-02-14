use crate::types::nature::Nature;
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, FromRepr)]
#[repr(u8)]
pub enum Stat {
    Hp = 1,
    Atk = 2,
    Def = 3,
    SpAtk = 4,
    SpDef = 5,
    Speed = 6,
}

impl Stat {
    /// Formula: https://bulbapedia.bulbagarden.net/wiki/Stat#Generation_III_onward
    pub fn compute(
        &self,
        level: u8,
        base: Stats<u8>,
        evs: Stats<u8>,
        ivs: Stats<u8>,
        nature: Nature,
    ) -> u16 {
        let base = base.get(*self) as u16;
        let level = level as u16;
        let ev = evs.get(*self) as f64;
        let iv = ivs.get(*self) as u16;
        let nature = if *self == nature.increased_stat() {
            1.1
        } else if *self == nature.decreased_stat() {
            0.9
        } else {
            1.0
        };

        let baseline =
            (((2 * base + iv + (ev / 4.0).floor() as u16) * level) as f64 / 100.0).floor() as u16;
        if *self == Stat::Hp {
            baseline + level + 10
        } else {
            ((baseline + 5) as f64 * nature).floor() as u16
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display, FromRepr)]
#[repr(u8)]
pub enum ExtendedStat {
    Hp = 1,
    Atk = 2,
    Def = 3,
    SpAtk = 4,
    SpDef = 5,
    Speed = 6,
    Accuracy = 7,
    Evasion = 8,
    Special = 9,
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    bitcode::Encode,
    bitcode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Stats<T> {
    pub hp: T,
    pub atk: T,
    pub def: T,
    pub sp_atk: T,
    pub sp_def: T,
    pub speed: T,
}

impl<T> Stats<T> {
    pub fn set(&mut self, stat: Stat, value: T) {
        match stat {
            Stat::Hp => self.hp = value,
            Stat::Atk => self.atk = value,
            Stat::Def => self.def = value,
            Stat::SpAtk => self.sp_atk = value,
            Stat::SpDef => self.sp_def = value,
            Stat::Speed => self.speed = value,
        }
    }

    pub fn get(&self, stat: Stat) -> T
    where
        T: Copy,
    {
        match stat {
            Stat::Hp => self.hp,
            Stat::Atk => self.atk,
            Stat::Def => self.def,
            Stat::SpAtk => self.sp_atk,
            Stat::SpDef => self.sp_def,
            Stat::Speed => self.speed,
        }
    }
}

impl Stats<u16> {
    pub fn compute(
        level: u8,
        base: Stats<u8>,
        evs: Stats<u8>,
        ivs: Stats<u8>,
        nature: Nature,
    ) -> Stats<u16> {
        Stats {
            hp: Stat::Hp.compute(level, base, evs, ivs, nature),
            atk: Stat::Atk.compute(level, base, evs, ivs, nature),
            def: Stat::Def.compute(level, base, evs, ivs, nature),
            sp_atk: Stat::SpAtk.compute(level, base, evs, ivs, nature),
            sp_def: Stat::SpDef.compute(level, base, evs, ivs, nature),
            speed: Stat::Speed.compute(level, base, evs, ivs, nature),
        }
    }
}
