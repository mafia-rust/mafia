use std::time::Duration;

use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use super::{role_list::{RoleList, RoleListEntry}, role::Role, player::{Player, PlayerIndex}, phase::PhaseType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings{
    pub role_list: RoleList,
    pub invesigator_results: InvestigatorResults,
    pub phase_times: PhaseTimeSettings,
    // pub excluded_roles: Vec<Role>,
}
impl Default for Settings{
    fn default() -> Self {
        Self { role_list: Default::default(), invesigator_results: Default::default(), phase_times: Default::default() }
    }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigatorResults{
    results: Vec<Vec<Role>>,
}
impl InvestigatorResults{
    pub fn get_result(&self, role: Role)->Option<Vec<Role>>{
        for result in self.results.iter(){
            if result.contains(&role) {
                let mut out_vec = result.clone();
                out_vec.shuffle(&mut rand::thread_rng());
                return Some(out_vec);    
            }
        }
        None
    }
}
impl Default for InvestigatorResults{
    fn default() -> Self {
        Self { 
            results: vec![
                vec![Role::Veteran, Role::Mafioso],
                vec![], //med
                vec![], //surv, vh. amne, medusa, psy
                vec![], //spy
                vec![Role::Sheriff],
                vec![], //fram vamp jest
                vec![Role::Consort],
                vec![Role::Doctor],
                vec![], //Invest
                vec![], //Bodyguard
            ] 
        }
    }
}

/*
Vigilante, Veteran, Mafioso, Pirate, or Ambusher.
Medium, Janitor, Retributionist, Necromancer, or Trapper.
Survivor, Vampire Hunter, Amnesiac, Medusa, or Psychic.
Spy, Blackmailer, Jailor, or Guardian Angel.
Sheriff, Executioner, Werewolf, or Poisoner.
Framer, Vampire, Jester, or Hex Master.
Lookout, Forger, Juggernaut, or Coven Leader.
Escort, Transporter, Consort, or Hypnotist.
Doctor, Disguiser, Serial Killer, or Potion Master.
Investigator, Consigliere, Mayor, Tracker, or Plaguebearer.
Bodyguard, Godfather, Arsonist, or Crusader.

 */