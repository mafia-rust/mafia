
use rand::seq::IndexedRandom;
use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::components::confused::Confused;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{
    ControllerID, ControllerParametersMap,
    GetClientRoleState, Priority, Role, RoleStateImpl
};



#[derive(Clone, Debug, Default)]
pub struct Cop {
    target_protected_ref: Option<PlayerReference>
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Cop {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() == 1 {return}

        match priority {
            Priority::Heal => {
                if Confused::is_confused(game, actor_ref){return;}

                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                let target_ref = visit.target;
                
                target_ref.increase_defense_to(game, DefensePower::Protection);
                actor_ref.set_role_state(game, Cop {target_protected_ref: Some(target_ref)});
            }
            Priority::Kill => {
                
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                
                let Some(visit) = actor_visits.first() else {return};
                if Confused::is_confused(game, actor_ref){
                    actor_ref.push_night_message(game,ChatMessageVariant::SomeoneSurvivedYourAttack);
                    return;
                }

                let target_ref = visit.target;

                let player_to_attack =
                if let Some(non_town_visitor) = PlayerReference::all_players(game)
                    .filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref &&
                        !other_player_ref.win_condition(game).is_loyalist_for(GameConclusion::Town) &&
                        target_ref.all_night_visitors_cloned(game).contains(other_player_ref)
                    ).collect::<Vec<PlayerReference>>()
                    .choose(&mut rand::rng())
                    .copied(){
                    Some(non_town_visitor)
                } else {
                    PlayerReference::all_players(game)
                        .filter(|other_player_ref|
                            other_player_ref.alive(game) &&
                            *other_player_ref != actor_ref &&
                            //this is in the filter rather than being the Vec the players are filtered from to prevent repeat players
                            target_ref.all_night_visitors_cloned(game).contains(other_player_ref)
                        ).collect::<Vec<PlayerReference>>()
                        .choose(&mut rand::rng())
                        .copied()
                };
                
                if let Some(player_to_attack) = player_to_attack{
                    player_to_attack.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Cop), AttackPower::Basic, false);
                }
            }
            Priority::Investigative => {
                if let Some(target_protected_ref) = self.target_protected_ref {
                    if target_protected_ref.night_attacked(game){
                        
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        target_protected_ref.push_night_message(game, ChatMessageVariant::YouWereProtected);
                    }
                }
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            !(game.day_number() > 1),
            ControllerID::role(actor_ref, Role::Cop, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Cop, 0),
            false
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase != PhaseType::Night {return;}
        actor_ref.set_role_state(game, Cop {target_protected_ref: None});
    }
}
impl GetClientRoleState<ClientRoleState> for Cop {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}