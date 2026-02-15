use crate::battle::config::BattleConfig;
use crate::battle::types::side::BattleSide;
use crate::battle::types::team::BattleTeam;
use crate::battle::Battle;
use crate::data::Data;
use crate::error::PkmnResult;
use crate::storage::team::StoredTeam;
use std::sync::Arc;

pub struct BattleBuilder {
    data: Arc<Data>,
    config: BattleConfig,
    side_a: Vec<BattleTeam>,
    side_b: Vec<BattleTeam>,
    rng: Box<dyn rand::Rng>,
}

impl BattleBuilder {
    pub fn new(data: &Arc<Data>) -> Self {
        Self {
            data: data.clone(),
            config: BattleConfig::default(),
            side_a: Vec::new(),
            side_b: Vec::new(),
            rng: Box::new(rand::rng()),
        }
    }

    pub fn build(self) -> PkmnResult<Battle> {
        Ok(Battle {
            data: self.data,
            config: self.config,
            side_a: self.side_a,
            side_b: self.side_b,
            actions: Vec::new(),
            action_priorities: Vec::new(),
            events: Vec::new(),
            rng: self.rng,
        })
    }

    pub fn add_team(mut self, side: BattleSide, team: &StoredTeam) -> PkmnResult<Self> {
        let battle_team = BattleTeam::new(self.data.as_ref(), team, &self.config)?;
        match side {
            BattleSide::A => self.side_a.push(battle_team),
            BattleSide::B => self.side_b.push(battle_team),
        };
        Ok(self)
    }

    pub fn add_team_a(self, team: &StoredTeam) -> PkmnResult<Self> {
        self.add_team(BattleSide::A, team)
    }

    pub fn add_team_b(self, team: &StoredTeam) -> PkmnResult<Self> {
        self.add_team(BattleSide::B, team)
    }
}
