use serde::{Deserialize, Serialize};

use crate::game::attack_power::AttackPower;
use crate::game::attack_power::DefensePower;
use crate::game::resolution_state::ResolutionState;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, Role, RoleState, RoleStateImpl};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Marksman {
    state: MarksmanState
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    pub marks: Vec<PlayerReference>,
    pub camps: Vec<PlayerReference>
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub(self) enum MarksmanState{
    NotLoaded,
    Marks{
        marks: Vec<PlayerReference>,
        camps: Vec<PlayerReference>
    },
    ShotTownie
}
impl MarksmanState{
    fn no_marks(&self)->bool{
        self.marks().is_empty()
    }
    fn marks(&self)->Vec<PlayerReference> {
        if let Self::Marks{marks, ..} = self {
            marks.clone()
        }else{
            Vec::new()
        }
    }
    fn camps(&self)->Vec<PlayerReference> {
        if let Self::Marks{camps, ..} = self {
            camps.clone()
        }else{
            Vec::new()
        }
    }
}

impl Default for Marksman {
    fn default() -> Self {
        Self { state: MarksmanState::NotLoaded }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Marksman {
    type ClientRoleState = Marksman;
    type RoleActionChoice = RoleActionChoice;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
    
        match priority {
            Priority::Kill => {
                let visiting_players: Vec<_> = actor_ref
                    .night_visits(game)
                    .into_iter()
                    .flat_map(|p|p.target.all_visitors(game))
                    .collect();

                for mark in self.state.marks().into_iter() {
                    
                    if !visiting_players.contains(&mark) {continue};
                    
                    let killed = mark.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Marksman), AttackPower::Basic, false);

                    if killed && mark.win_condition(game).requires_only_this_resolution_state(ResolutionState::Town) {
                        self.state = MarksmanState::ShotTownie;
                    }
                }
                
                actor_ref.set_role_state(game, RoleState::Marksman(self));
            },
            _ => {}
        }

    }
    fn on_role_action(self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != PhaseType::Night {return};
        let MarksmanState::Marks { marks, camps } = self.state else {return};
        
        let (marks, camps) = if 
            super::common_role::default_action_choice_boolean_is_valid(game, actor_ref) &&
            validate_marks(game, actor_ref, &action_choice.marks) && 
            validate_camps(game, actor_ref, &action_choice.camps)
        {
            (marks, camps)
        }else{
            (Vec::new(), Vec::new())
        };
        actor_ref.set_role_state(game, RoleState::Marksman(Marksman{
            state: MarksmanState::Marks { marks, camps }
        }));
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        if self.state.camps().len() <= 3 && !self.state.no_marks(){
            self.state.camps().into_iter().map(|p|
                Visit{ target: p, attack: false }
            ).collect()
        }else{
            vec![]
        }
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if matches!(phase, PhaseType::Night|PhaseType::Obituary) && game.day_number() != 1 {
            actor_ref.set_role_state(game, 
                RoleState::Marksman(Marksman{
                    state:MarksmanState::Marks { marks: Vec::new(), camps: Vec::new() }
                })
            )
        }
    }
}

fn validate_marks(game: &Game, actor: PlayerReference, marks: &Vec<PlayerReference>)->bool{
    marks.len() <= 3 &&
    marks.iter().all(|p|p.alive(game) && *p != actor)
}
fn validate_camps(game: &Game, actor: PlayerReference, camps: &Vec<PlayerReference>)->bool{
    camps.len() <= 3 &&
    camps.iter().all(|p|p.alive(game) && *p != actor)
}