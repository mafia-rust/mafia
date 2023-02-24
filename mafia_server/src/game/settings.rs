use std::time::Duration;

use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use super::{role_list::{RoleList, RoleListEntry}, role::Role, player::{Player, PlayerIndex}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings{
    pub role_list: RoleList,

    pub invesigator_results: InvestigatorResults,

    pub phase_times: PhaseTimeSettings,
    // pub excluded_roles: Vec<Role>,
}
impl Settings{
    pub fn new(player_count : PlayerIndex)->Self {
        Self { 
            role_list: RoleList{
                role_list: vec![RoleListEntry::Any],
            }, 
            invesigator_results: InvestigatorResults::default(),
            phase_times: PhaseTimeSettings { 
                morning: Duration::from_secs(5), 
                discussion: Duration::from_secs(45), 
                voting: Duration::from_secs(30), 
                testimony: Duration::from_secs(20), 
                judgement: Duration::from_secs(20), 
                evening: Duration::from_secs(10), 
                night: Duration::from_secs(37) 
            },
        }
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