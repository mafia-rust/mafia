use rand::thread_rng;
use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::player_group::PlayerGroup;
use crate::game::grave::{Grave, GraveReference};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use super::{Priority, RoleStateImpl};
use rand::prelude::SliceRandom;


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scarecrow;

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Scarecrow {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Ward {return;}
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;

            let mut blocked_players = target_ref.ward(game);
            blocked_players.shuffle(&mut thread_rng());

            let message = ChatMessageVariant::ScarecrowResult { players:
                PlayerReference::ref_vec_to_index(blocked_players.as_slice())
            };

            for player_ref in blocked_players.iter(){
                actor_ref.insert_role_label(game, *player_ref);
            }
            
            actor_ref.push_night_message(game, message);
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, _actor_ref: PlayerReference) -> bool {
        PlayerReference::all_players(game).filter(|player_ref|{
            player_ref.alive(game) && player_ref.role(game).faction() == Faction::Town
        }).count() == 0
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        if actor_ref.get_won_game(game) && actor_ref.alive(game) {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}