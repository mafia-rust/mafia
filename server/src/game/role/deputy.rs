
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::{GraveKiller, Grave, GraveDeathCause, GraveRole};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use crate::packet::ToClientPacket;
use super::{Priority, RoleStateImpl, Role, RoleState};

pub(super) const SUSPICIOUS: bool = false;
pub(super) const WITCHABLE: bool = true;
pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownKilling;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Deputy {
    bullets_remaining: u8,  //TODO this entire role, powerfull attack, die function, cleaned, etc
}
impl Default for Deputy {
    fn default() -> Self {
        Self { bullets_remaining: 1}
    }
}
impl RoleStateImpl for Deputy {
    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {
        
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {

        for other_ref in PlayerReference::all_players(game){
            other_ref.insert_role_label(game, actor_ref, Role::Deputy);
        }
        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::DeputyShot { deputy_index: actor_ref.index(), shot_index: target_ref.index() });
        


        target_ref.set_alive(game, false);
        let mut new_grave = Grave::from_player_night(game, target_ref);
        new_grave.death_cause = GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Deputy)]);
        new_grave.role = GraveRole::Role(target_ref.role(game));
        game.graves.push(new_grave.clone());
        game.send_packet_to_all(ToClientPacket::AddGrave{grave: new_grave.clone()});
        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PlayerDied { grave: new_grave.clone() });
        if let Some(role) = new_grave.role.get_role(){
            for other_player_ref in PlayerReference::all_players(game){
                other_player_ref.insert_role_label(game, target_ref, role);
            }
        }



        if target_ref.role(game).faction_alignment().faction() == Faction::Town {
            actor_ref.set_alive(game, false);
            let mut new_grave = Grave::from_player_night(game, actor_ref);
            new_grave.death_cause = GraveDeathCause::Killers(vec![GraveKiller::Suicide]);
            new_grave.role = GraveRole::Role(Role::Deputy);
            game.graves.push(new_grave.clone());
            game.send_packet_to_all(ToClientPacket::AddGrave{grave: new_grave.clone()});
            game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PlayerDied { grave: new_grave.clone() });
            if let Some(role) = new_grave.role.get_role(){
                for other_player_ref in PlayerReference::all_players(game){
                    other_player_ref.insert_role_label(game, actor_ref, role);
                }
            }
        }

        actor_ref.set_role_state(game, RoleState::Deputy(Deputy{bullets_remaining:0}));
        
    }
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        self.bullets_remaining > 0 && actor_ref != target_ref && target_ref.alive(game) && actor_ref.alive(game)
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self,  _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {

    }
    fn on_role_creation(self,  game: &mut Game, actor_ref: PlayerReference) {
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
}