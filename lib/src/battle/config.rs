pub struct BattleConfig {
    pub active_slots_per_team: usize,
}

impl Default for BattleConfig {
    fn default() -> Self {
        Self {
            active_slots_per_team: 1,
        }
    }
}
