use std::{collections::HashSet, time::Duration};

use serde::{Serialize, Deserialize};

use super::{modifiers::ModifierType, phase::PhaseType, role::Role, role_list::RoleList};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings{
    pub role_list: RoleList,
    pub phase_times: PhaseTimeSettings,
    pub enabled_roles: HashSet<Role>,
    pub enabled_modifiers: HashSet<ModifierType>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseTimeSettings{
    pub briefing: u64,
    pub obituary: u64,
    pub discussion: u64,
    pub nomination: u64,
    pub testimony: u64,
    pub judgement: u64,
    pub final_words: u64,
    pub dusk: u64,
    pub night: u64,
}
impl PhaseTimeSettings {
    pub fn get_time_for(&self, phase: PhaseType) -> Duration {
        match phase {
            PhaseType::Briefing => Duration::from_secs(self.briefing),
            PhaseType::Discussion => Duration::from_secs(self.discussion),
            PhaseType::FinalWords => Duration::from_secs(self.final_words),
            PhaseType::Dusk => Duration::from_secs(self.dusk),
            PhaseType::Judgement => Duration::from_secs(self.judgement),
            PhaseType::Obituary => Duration::from_secs(self.obituary),
            PhaseType::Night => Duration::from_secs(self.night),
            PhaseType::Testimony => Duration::from_secs(self.testimony),
            PhaseType::Nomination => Duration::from_secs(self.nomination)
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
            obituary: 10,
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