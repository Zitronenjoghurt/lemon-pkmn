pub struct BattleConfig {
    pub active_slots_per_team: usize,
    pub critical_hit_multiplier: f64,
}

impl Default for BattleConfig {
    fn default() -> Self {
        Self {
            active_slots_per_team: 1,
            critical_hit_multiplier: 1.5,
        }
    }
}
