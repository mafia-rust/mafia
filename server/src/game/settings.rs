use std::time::Duration;

use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use super::{role_list::{RoleList, RoleListEntry}, role::Role, player::Player, phase::PhaseType};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings{
    pub role_list: RoleList,
    pub phase_times: PhaseTimeSettings,
    // pub excluded_roles: Vec<Role>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTimeSettings{
    pub morning: Duration,
    pub discussion: Duration,
    pub voting: Duration,
    pub testimony: Duration,
    pub judgement: Duration,
    pub evening: Duration,
    pub night: Duration,
}
impl PhaseTimeSettings {
    pub fn get_time_for(&self, phase: PhaseType) -> Duration {
        match phase {
            PhaseType::Discussion => self.discussion,
            PhaseType::Evening => self.evening,
            PhaseType::Judgement => self.judgement,
            PhaseType::Morning => self.morning,
            PhaseType::Night => self.night,
            PhaseType::Testimony => self.testimony,
            PhaseType::Voting => self.voting
        }
    }
}
impl Default for PhaseTimeSettings{
    fn default() -> Self {
        Self { 
            morning: Duration::from_secs(5), 
            discussion: Duration::from_secs(45), 
            voting: Duration::from_secs(30), 
            testimony: Duration::from_secs(20), 
            judgement: Duration::from_secs(20), 
            evening: Duration::from_secs(7), 
            night: Duration::from_secs(37) 
        }
    }
}