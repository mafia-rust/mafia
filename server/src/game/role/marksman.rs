
use serde::Serialize;

use crate::game::chat::ChatGroup;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl, Role, RoleState};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Marksman {
    state: MarksmanState
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub(self) enum MarksmanState{
    NotLoaded,
    Marks{
        marks: MarksmanMarks
    },
    ShotTownie
}
impl MarksmanState{
    fn no_marks(&self)->bool{
        !matches!(
            self,
            MarksmanState::Marks{marks: MarksmanMarks::One{..}} |
            MarksmanState::Marks{marks: MarksmanMarks::Two{..}}
        )
    }
    fn marks(&self)->Box<[PlayerReference]> {
        if let Self::Marks{marks} = self {
            marks.marks()
        }else{
            Box::new([])   
        }
    }
    /// This function will mark an unmarked player or un-mark a marked player
    /// if the action is invalid, then it will do nothing
    fn toggle_mark(&mut self, p: PlayerReference){
        let Self::Marks { marks } = self else {return};
        *marks = match marks {
            MarksmanMarks::None => {
                MarksmanMarks::One { a: p }
            }
            MarksmanMarks::One {a} => {
                if *a != p {
                    MarksmanMarks::Two { a: *a, b: p }
                }else{
                    MarksmanMarks::None
                }
            }
            MarksmanMarks::Two { a, b } => {
                if p == *a {
                    MarksmanMarks::One { a: *b }
                }else if p == *b {
                    MarksmanMarks::One { a: *a }
                }else{
                    MarksmanMarks::Two { a: *a, b: *b }
                }
            }
        };
    }
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum MarksmanMarks{
    None,
    One{
        a: PlayerReference
    },
    Two{
        a: PlayerReference, 
        b: PlayerReference
    }
}
impl MarksmanMarks{
    fn marks(&self)->Box<[PlayerReference]> {
        match self {
            MarksmanMarks::None => {Box::new([])},
            MarksmanMarks::One{a} => {Box::new([*a])},
            MarksmanMarks::Two{a,b} => {Box::new([*a,*b])}
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

impl RoleStateImpl for Marksman {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    

    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
    
        match priority{
            Priority::Kill => {
                let visiting_players: Vec<_> = actor_ref
                    .night_visits(game)
                    .into_iter()
                    .flat_map(|p|p.target.all_visitors(game))
                    .collect();

                for mark in self.state.marks().into_iter() {
                    
                    if !visiting_players.contains(&mark) {continue};
                    
                    let killed = mark.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Marksman), 1, false);

                    if killed && mark.role(game).faction() == Faction::Town {
                        self.state = MarksmanState::ShotTownie;
                    }
                }
                
                actor_ref.set_role_state(game, RoleState::Marksman(self));
            },
            _ => {}
        }

    }
    fn do_day_action(mut self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        self.state.toggle_mark(target_ref);
        actor_ref.set_role_state(game, RoleState::Marksman(self))
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let chosen_targets = actor_ref.chosen_targets(game);
        
        !self.state.no_marks() &&
        actor_ref != target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) && 
        ((
            chosen_targets.is_empty()
        ) || (
            chosen_targets.len() == 1 &&
            Some(target_ref) != chosen_targets.first().copied()
        ))
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        game.current_phase().is_night() &&
        target_ref.alive(game) &&
        if let MarksmanState::Marks {..} = self.state {true}else{false} &&
        actor_ref != target_ref
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{ target: target_refs[0], attack: false }, 
                Visit{ target: target_refs[1], attack: false }
            ]
        } else if target_refs.len() == 1 {
            vec![
                Visit{ target: target_refs[0], attack: false }
            ]
        } else {
            Vec::new()
        }
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase == PhaseType::Night && game.day_number()!=1{
            actor_ref.set_role_state(game, 
                RoleState::Marksman(Marksman{
                    state:MarksmanState::Marks { marks: MarksmanMarks::None }
                })
            )
        }
    }
    fn on_role_creation(self,  _game: &mut Game, _actor_ref: PlayerReference) {
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave: crate::game::grave::GraveReference) {
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}