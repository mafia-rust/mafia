use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, components::confused::Confused};
use crate::game::chat::ChatMessageVariant;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Spy;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum SpyBug{
    Silenced, 
    Roleblocked, Wardblocked,
    Protected, 
    Transported, Possessed
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Spy {
    type ClientRoleState = Spy;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Investigative => {
                if actor_ref.night_blocked(game) {return;}
                if actor_ref.ability_deactivated_from_death(game) {return;}

                let mut mafia_visits = vec![];
                for other_player in PlayerReference::all_players(game){
                    if !InsiderGroupID::Mafia.is_player_in_revealed_group(game, other_player) {continue}
                    mafia_visits.append(
                        &mut other_player.tracker_seen_visits(game)
                            .iter()
                            .map(|v|v.target.index())
                            .collect()
                    );
                }
                mafia_visits.shuffle(&mut rand::rng());
                
                actor_ref.push_night_message(game, ChatMessageVariant::SpyMafiaVisit { players: mafia_visits });               
            },
            Priority::SpyBug => {
                if Confused::is_confused(game, actor_ref) {return}

                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};

                for message in visit.target.night_messages(game).clone(){
                    if let Some(message) = match message{
                        ChatMessageVariant::Silenced => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Silenced }),
                        ChatMessageVariant::RoleBlocked { immune: _ } => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Roleblocked }),
                        ChatMessageVariant::YouWereProtected => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Protected }),
                        ChatMessageVariant::Transported => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Transported }),
                        ChatMessageVariant::YouWerePossessed { immune: _ } => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Possessed }),
                        ChatMessageVariant::Wardblocked => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Wardblocked }),
                        _ => None
                    }{
                        actor_ref.push_night_message(game, message);
                    }
                };
            }
            _=>{}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Spy, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Spy, 0),
            false
        )
    }
}