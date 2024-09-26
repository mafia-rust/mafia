use rand::thread_rng;
use rand::prelude::SliceRandom;
use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl};

#[derive(Clone, Serialize, Debug, Default)]
pub struct Lookout;

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Lookout {
    type ClientRoleState = Lookout;
    type RoleActionChoice = super::common_role::CommonRoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            
            let mut seen_players: Vec<PlayerReference> = visit.target.appeared_visitors(game).into_iter().filter(|p|actor_ref!=*p).collect();
            seen_players.shuffle(&mut thread_rng());

            let message = ChatMessageVariant::LookoutResult { players:
                PlayerReference::ref_vec_to_index(seen_players.as_slice())
            };
            
            actor_ref.push_night_message(game, message);
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
}