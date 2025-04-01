use std::time::Duration;

use serde::{Serialize, Deserialize};

use crate::vec_set::VecSet;

use super::{modifiers::ModifierType, phase::PhaseType, role::Role, role_list::RoleList};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings{
    pub role_list: RoleList,
    pub phase_times: PhaseTimeSettings,
    pub enabled_roles: VecSet<Role>,
    pub enabled_modifiers: VecSet<ModifierType>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseTimeSettings{
    pub briefing: u16,
    pub obituary: u16,
    pub discussion: u16,
    pub nomination: u16,
    pub testimony: u16,
    pub judgement: u16,
    pub final_words: u16,
    pub dusk: u16,
    pub night: u16,
}
impl PhaseTimeSettings {
    pub fn get_time_for(&self, phase: PhaseType) -> Option<Duration> {
        match phase {
            PhaseType::Briefing => Some(Duration::from_secs(self.briefing as u64)),
            PhaseType::Discussion => Some(Duration::from_secs(self.discussion as u64)),
            PhaseType::FinalWords => Some(Duration::from_secs(self.final_words as u64)),
            PhaseType::Dusk => Some(Duration::from_secs(self.dusk as u64)),
            PhaseType::Judgement => Some(Duration::from_secs(self.judgement as u64)),
            PhaseType::Obituary => Some(Duration::from_secs(self.obituary as u64)),
            PhaseType::Night => Some(Duration::from_secs(self.night as u64)),
            PhaseType::Testimony => Some(Duration::from_secs(self.testimony as u64)),
            PhaseType::Nomination => Some(Duration::from_secs(self.nomination as u64)),
            PhaseType::Recess => None
        }
    }
    pub fn game_ends_instantly(&self)->bool{
        [self.obituary, self.discussion, self.nomination, self.night, self.dusk].iter().all(|t| *t == 0)
    }
}
impl Default for PhaseTimeSettings{
    fn default() -> Self {
        Self{
            briefing: 45,
            obituary: 60,
            discussion: 120,
            nomination: 120,
            testimony: 30,
            judgement: 60,
            final_words: 30,
            dusk: 30,
            night: 60,
        }
    }
}