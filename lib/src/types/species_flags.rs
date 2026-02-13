use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct SpeciesFlags: u8 {
        const DEFAULT_FORM = 0b0000_0001;
        const BATTLE_ONLY = 0b0000_0010;
        const FORM_SWITCHABLE = 0b0000_0100;
        const MEGA = 0b0000_1000;
        const GMAX = 0b0001_0000;
        const LEGENDARY = 0b0010_0000;
        const MYTHICAL = 0b0100_0000;
    }
}
