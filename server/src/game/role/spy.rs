use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Spy;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum SpyBug{
    Silenced, Roleblocked, Protected, Transported, Possessed
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Spy {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Investigative => {
                if actor_ref.night_roleblocked(game) {return;}

                let mut mafia_visits = vec![];
                for other_player in PlayerReference::all_players(game){
                    if other_player.role(game).faction() == Faction::Mafia{
                        mafia_visits.append(&mut other_player.night_visits(game).iter().map(|v|v.target.index()).collect());
                    }
                }
                mafia_visits.shuffle(&mut rand::thread_rng());
                
                actor_ref.push_night_message(game, ChatMessageVariant::SpyMafiaVisit { players: mafia_visits });               
            },
            Priority::SpyBug => {
                let Some(visit) = actor_ref.night_visits(game).first()else{return};
            
                if visit.target.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessageVariant::TargetJailed );
                    return
                }

                for message in visit.target.night_messages(game).clone(){
                    if let Some(message) = match message{
                        ChatMessageVariant::Silenced => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Silenced }),
                        ChatMessageVariant::RoleBlocked { immune: _ } => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Roleblocked }),
                        ChatMessageVariant::YouWereProtected => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Protected }),
                        ChatMessageVariant::Transported => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Transported }),
                        ChatMessageVariant::YouWerePossessed { immune: _ } => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Possessed }),
                        _ => None
                    }{
                        actor_ref.push_night_message(game, message);
                    }
                };
            },
            Priority::SpyCultistCount => {
                actor_ref.push_night_message(game, ChatMessageVariant::SpyCultistCount { count: 
                    PlayerReference::all_players(game).filter(|p|
                        p.role(game).faction() == Faction::Cult && p.alive(game)
                    ).count() as u8
                }); 
            }
            _=>{}
        }
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Night => {
                //if there are any cult alive, tell the spy if apostle can convert
                if PlayerReference::all_players(game).any(|p|p.role(game).faction() == Faction::Cult){
                    if game.teams.cult().can_convert_tonight(game) {
                        actor_ref.add_private_chat_message(game,
                            ChatMessageVariant::ApostleCanConvertTonight
                        )
                    }else{
                        actor_ref.add_private_chat_message(game,
                            ChatMessageVariant::ApostleCantConvertTonight
                        )
                    }
                }
            },
            _=>{}
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}