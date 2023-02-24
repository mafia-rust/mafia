use std::time::Duration;

use rand::seq::SliceRandom;

use super::{role_list::RoleList, role::Role};


pub struct Settings{
    pub role_list: RoleList,

    // pub invesigator_results: TODO

    pub phase_times: PhaseTimeSettings,
    // pub excluded_roles: Vec<Role>,
}
pub struct PhaseTimeSettings{
    pub morning: Duration,
    pub discussion: Duration,
    pub voting: Duration,
    pub testimony: Duration,
    pub judgement: Duration,
    pub evening: Duration,
    pub night: Duration,
}
pub struct InvestigatorResults{
    results: Vec<Vec<Role>>,
}
impl InvestigatorResults{
    pub fn get_result(&self, role: Role)->Option<Vec<Role>>{
        for result in self.results.iter(){
            if result.contains(&role) {
                let out_vec = result.clone();
                out_vec.shuffle(&mut rand::thread_rng());
                return Some(out_vec);    
            }
        }
        None
    }
    
}