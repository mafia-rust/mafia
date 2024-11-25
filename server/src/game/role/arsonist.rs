use serde::Serialize;

use crate::game::{attack_power::DefensePower, components::arsonist_doused::ArsonistDoused};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Arsonist;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Arsonist {
    type ClientRoleState = Arsonist;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        

        match priority {
            Priority::Deception => {
                //douse target
                let actor_visits = actor_ref.night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    let target_ref = visit.target;
                    ArsonistDoused::douse(game, target_ref);
                }
                
                //douse all visitors
                for other_player_ref in PlayerReference::all_players(game)
                    .filter(|other_player_ref|
                        *other_player_ref != actor_ref &&
                        other_player_ref.night_visits_cloned(game)
                            .iter()
                            .any(|v|v.target==actor_ref)
                    ).collect::<Vec<PlayerReference>>()
                {   
                    ArsonistDoused::douse(game, other_player_ref);
                }
            },
            Priority::Kill => {
                let actor_visits = actor_ref.night_visits_cloned(game);             
                if let Some(visit) = actor_visits.first(){
                    if actor_ref == visit.target{
                        ArsonistDoused::ignite(game, actor_ref);
                    }
                }
            }
            _ => {}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game) &&
        target_ref.alive(game)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        ArsonistDoused::clean_doused(game, actor_ref);
    }
}
