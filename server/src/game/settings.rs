use std::time::Duration;

use serde::{Serialize, Deserialize};

use super::{role_list::{RoleList, RoleOutline}, phase::PhaseType};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings{
    pub role_list: RoleList,
    pub phase_times: PhaseTimeSettings,
    pub excluded_roles: Vec<RoleOutline>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTimeSettings{
    pub morning: u64,
    pub discussion: u64,
    pub voting: u64,
    pub testimony: u64,
    pub judgement: u64,
    pub evening: u64,
    pub night: u64,
}
impl PhaseTimeSettings {
    pub fn get_time_for(&self, phase: PhaseType) -> Duration {
        match phase {
            PhaseType::Discussion => Duration::from_secs(self.discussion),
            PhaseType::Evening => Duration::from_secs(self.evening),
            PhaseType::Judgement => Duration::from_secs(self.judgement),
            PhaseType::Morning => Duration::from_secs(self.morning),
            PhaseType::Night => Duration::from_secs(self.night),
            PhaseType::Testimony => Duration::from_secs(self.testimony),
            PhaseType::Voting => Duration::from_secs(self.voting)
        }
    }
}
impl Default for PhaseTimeSettings{
    fn default() -> Self {
        Self{
            morning: 5, 
            discussion: 45, 
            voting: 30, 
            testimony: 20, 
            judgement: 20, 
            evening: 7, 
            night: 37 
        }
    }
}