
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleState, RoleStateImpl, common_role, Role};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCK_IMMUNE: bool = false;
pub(super) const CONTROL_IMMUNE: bool = false;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownProtective;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;

#[derive(Clone, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Crusader {
    target_protected_ref: Option<PlayerReference>
}

impl RoleStateImpl for Crusader {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}
    
        match priority {
            Priority::Heal => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;
                if target_ref.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                    return
                }

                target_ref.increase_defense_to(game, 2);
                actor_ref.set_role_state(game, RoleState::Crusader(Crusader {target_protected_ref: Some(target_ref)}));
            }
            Priority::Kill => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;
                if target_ref.night_jailed(game){
                    actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                    return
                }

                let non_town_visitor: Option<PlayerReference> = PlayerReference::all_players(game)
                    .into_iter()
                    .filter(|other_player_ref|
                        *other_player_ref != actor_ref &&
                        other_player_ref.role(game).faction_alignment().faction() != Faction::Town &&
                        other_player_ref.night_visits(game)
                            .iter()
                            .any(|v|!v.astral&&v.target==target_ref)
                    ).collect::<Vec<PlayerReference>>()
                    .choose(&mut rand::thread_rng())
                    .copied();

                if let Some(non_town_visitor) = non_town_visitor{
                    non_town_visitor.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Crusader), 1);
                }else{
                    let town_visitor: Option<PlayerReference> = PlayerReference::all_players(game)
                        .into_iter()
                        .filter(|other_player_ref|
                            *other_player_ref != actor_ref &&
                            other_player_ref.night_visits(game)
                                .iter()
                                .any(|v|!v.astral&&v.target==target_ref)
                        ).collect::<Vec<PlayerReference>>()
                        .choose(&mut rand::thread_rng())
                        .copied();

                    if let Some(town_visitor) = town_visitor{
                        town_visitor.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Crusader), 1);
                    }
                }
            }
            Priority::Investigative => {
                if let Some(target_protected_ref) = self.target_protected_ref {
                    if target_protected_ref.night_attacked(game){
                        
                        actor_ref.push_night_message(game, ChatMessage::ProtectedYou);
                        target_protected_ref.push_night_message(game, ChatMessage::ProtectedYou);
                    }
                }
            }
            _ => {}
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        common_role::can_night_target(game, actor_ref, target_ref)
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
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase != PhaseType::Night {return;}
        actor_ref.set_role_state(game, RoleState::Crusader(Crusader {target_protected_ref: None}));
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
}