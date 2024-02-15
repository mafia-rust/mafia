use std::vec;

use serde::Serialize;

use crate::game::chat::ChatMessage;
use crate::game::grave::GraveKiller;
use crate::game::chat::ChatGroup;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::team::{Team, Vampires};
use crate::game::visit::Visit;
use crate::game::Game;
use super::renfield::Renfield;
use super::{Priority, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Dracula;

pub(super) const FACTION: Faction = Faction::Vampire;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

impl RoleStateImpl for Dracula {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {Some(Team::Vampires)}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        let mut vampires = game.teams.vampires().clone();
        match priority {
            Priority::Kill => {
                if vampires.ordered_vampires.len() != 1 {return}
                if vampires.can_convert_tonight(game) {return}

                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;

                if target_ref.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                    return
                }
        
                target_ref.try_night_kill(
                    actor_ref, game, GraveKiller::Faction(Faction::Vampire), 1, false
                );
            }
            Priority::Convert => {
                if !vampires.can_convert_tonight(game) {return}
                
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;

                if target_ref.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                    return
                }

                if target_ref.night_defense(game) > 0 {
                    actor_ref.push_night_message(game, ChatMessage::YourConvertFailed);
                    return
                }                

                target_ref.set_role(game, RoleState::Renfield(Renfield));

                vampires.sacrifices_needed = Some(Vampires::SACRIFICES_NEEDED);
                game.teams.set_vampires(vampires);
            }
            _ => {}
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {

        let vampires = game.teams.vampires().clone();
        let can_kill = vampires.ordered_vampires.len() == 1 && !vampires.can_convert_tonight(game);
        let can_convert = vampires.can_convert_tonight(game);

        if !can_convert && !can_kill {return false}

        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Vampire])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
