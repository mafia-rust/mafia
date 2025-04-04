use serde::Serialize;

use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::{attack_power::DefensePower, components::arsonist_doused::ArsonistDoused};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Arsonist;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Arsonist {
    type ClientRoleState = Arsonist;
    fn on_midnight(self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        match priority {
            OnMidnightPriority::Deception => {
                //douse target
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    let target_ref = visit.target;
                    ArsonistDoused::douse(game, target_ref);
                }
                
                //douse all visitors
                for other_player_ref in actor_ref.all_night_visitors_cloned(game)
                    .into_iter()
                    .filter(|other_player_ref| *other_player_ref != actor_ref)
                    .collect::<Vec<PlayerReference>>()
                {
                    ArsonistDoused::douse(game, other_player_ref);
                }
            },
            OnMidnightPriority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);             
                if let Some(visit) = actor_visits.first(){
                    if actor_ref == visit.target{
                        ArsonistDoused::ignite(game, actor_ref);
                    }
                }
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Arsonist, 0))
            .single_player_selection_typical(actor_ref, true, true)
            .night_typical(actor_ref)
            .add_grayed_out_condition(false)
            .build_map()
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Arsonist, 0),
            false
        )
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        ArsonistDoused::clean_doused(game, actor_ref);
    }
}
