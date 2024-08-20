use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::resolution_state::ResolutionState;
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
        marks: Vec<PlayerReference>
    },
    ShotTownie
}
impl MarksmanState{
    fn no_marks(&self)->bool{
        self.marks().is_empty()
    }
    fn marks(&self)->Vec<PlayerReference> {
        if let Self::Marks{marks} = self {
            marks.clone()
        }else{
            Vec::new()
        }
    }
    /// This function will mark an unmarked player or un-mark a marked player
    /// if the action is invalid, then it will do nothing
    fn toggle_mark(&mut self, p: PlayerReference){
        let Self::Marks { marks } = self else {return};
        if marks.contains(&p) {
            marks.retain(|x| x != &p);
        } else if marks.len() < 3 {
            marks.push(p);
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
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Marksman {
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
                    
                    let killed = mark.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Marksman), 1, false);

                    if killed && ResolutionState::requires_only_this_resolution_state(game, mark, ResolutionState::Town) {
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
        if self.state.marks().len() == 0 {
            actor_ref.set_selection(game, vec![]);
        }
        actor_ref.add_private_chat_message(game, 
            ChatMessageVariant::MarksmanChosenMarks { marks: PlayerReference::ref_vec_to_index(&self.state.marks()) }
        );
        actor_ref.set_role_state(game, RoleState::Marksman(self));
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let selection = actor_ref.selection(game);
        
        !self.state.no_marks() &&
        actor_ref != target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) && 
        (
            selection.len() < 3 &&
            !selection.contains(&target_ref)
        )
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        game.current_phase().is_night() &&
        actor_ref != target_ref &&
        actor_ref.alive(game) &&
        !actor_ref.night_jailed(game) &&
        target_ref.alive(game) &&
        matches!(self.state, MarksmanState::Marks { .. }) &&
        ((
            self.state.marks().len() == 3 &&
            self.state.marks().contains(&target_ref)
        ) || (
            self.state.marks().len() < 3
        ))
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() <= 3 {
            target_refs.into_iter().map(|p|
                Visit{ target: p, attack: false }
            ).collect()
        }else{
            vec![]
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
        if matches!(phase, PhaseType::Night|PhaseType::Obituary) && game.day_number() != 1 {
            actor_ref.set_role_state(game, 
                RoleState::Marksman(Marksman{
                    state:MarksmanState::Marks { marks: Vec::new() }
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