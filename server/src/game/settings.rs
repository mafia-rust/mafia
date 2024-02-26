use std::time::Duration;

use serde::{Serialize, Deserialize};

use super::{phase::PhaseType, role::Role, role_list::RoleList};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings{
    pub role_list: RoleList,
    pub phase_times: PhaseTimeSettings,
    pub excluded_roles: Vec<Role>,
}
impl Default for Settings{
    fn default() -> Self {
        Self{
            role_list: RoleList::default(),
            phase_times: PhaseTimeSettings::default(),
            excluded_roles: vec![Role::Jailor, Role::Bodyguard, Role::Mafioso, Role::Martyr]
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTimeSettings{
    pub briefing: u64,
    pub morning: u64,
    pub discussion: u64,
    pub voting: u64,
    pub testimony: u64,
    pub judgement: u64,
    pub evening: u64,
    pub dusk: u64,
    pub night: u64,
}
impl PhaseTimeSettings {
    pub fn get_time_for(&self, phase: PhaseType) -> Duration {
        match phase {
            PhaseType::Briefing => Duration::from_secs(self.briefing),
            PhaseType::Discussion => Duration::from_secs(self.discussion),
            PhaseType::Evening => Duration::from_secs(self.evening),
            PhaseType::Dusk => Duration::from_secs(self.dusk),
            PhaseType::Judgement => Duration::from_secs(self.judgement),
            PhaseType::Morning => Duration::from_secs(self.morning),
            PhaseType::Night => Duration::from_secs(self.night),
            PhaseType::Testimony => Duration::from_secs(self.testimony),
            PhaseType::Voting => Duration::from_secs(self.voting)
        }
    }
    pub fn game_ends_instantly(&self)->bool{
        [self.morning, self.discussion, self.voting, self.night, self.dusk].iter().all(|t| *t == 0)
    }
}
impl Default for PhaseTimeSettings{
    fn default() -> Self {
        Self{
            briefing: 20,
            morning: 10,
            discussion: 100,
            voting: 60,
            testimony: 30,
            judgement: 30,
            evening: 7,
            dusk: 7,
            night: 45,
        }
    }
}