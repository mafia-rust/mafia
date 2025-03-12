
use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::components::night_visits::NightVisits;
use crate::game::grave::{GraveInformation, GraveKiller, GraveReference};
use crate::game::chat::ChatMessageVariant;
use crate::game::player::PlayerReference;


use crate::game::visit::Visit;
use crate::game::Game;
use super::{
	ControllerID, ControllerParametersMap, GetClientRoleState, Priority, Role, RoleStateImpl
};

#[derive(Clone, Debug, Default)]
pub struct Coward {
	pub obscure: Option<PlayerReference>
}

#[derive(Clone, Serialize, Debug)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Coward {
    type ClientRoleState = ClientRoleState;

    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
    	if priority != Priority::Heal {return;}
    
    	let actor_visits = actor_ref.untagged_night_visits_cloned(game);
    
    	if let Some(target) = actor_visits.first(){
	       	actor_ref.push_night_message(game, 
	            ChatMessageVariant::CowardHid
	        );
			
			let mut saved : bool = false;
			let new_visits = NightVisits::all_visits(game).into_iter().copied().collect::<Vec<_>>();
			for mut visit in new_visits {
                if 
                    visit.attack &&
                    visit.target == actor_ref &&
                    visit.visitor != actor_ref
                {
                	visit.target = target.target;
                 	saved = true;
                }
            }
            
            for player_ref in PlayerReference::all_players(game){
                if player_ref == actor_ref {continue;}
                if player_ref.role(game) == Role::Transporter {continue;}
    
    
                let new_visits = player_ref.all_night_visits_cloned(game).clone().into_iter().map(|mut v|{
                    if
                    	v.attack &&
                    	v.target == actor_ref &&
                    	v.visitor != actor_ref {
                     	saved = true;
                        v.target = target.target;
                    }
                    v
                }).collect();
                player_ref.set_night_visits(game, new_visits);
                
                let obscure = Some(target.target);
                actor_ref.set_role_state(game, Coward{obscure});
            }
            
            if saved {
	           	actor_ref.push_night_message(game, 
		            ChatMessageVariant::CowardSaved
		        );
            }
        }
    }
    
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            true,
            true,
            false,
            ControllerID::role(actor_ref, Role::Coward, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Coward, 0),
            false
        )
    }
    
    fn on_grave_added(self, game: &mut Game, _: PlayerReference, grave_ref: GraveReference){
    	if let Some(player) = self.obscure {
        	if player == grave_ref.deref(game).player {
        	 	grave_ref.deref_mut(game).information = GraveInformation::Obscured;
         	}
        }
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: crate::game::phase::PhaseType) {
    	if let crate::game::phase::PhaseType::Night = _phase {
	   		let obscure = None;
	     	_actor_ref.set_role_state(_game, Coward{obscure});
     	}
    }
}

impl GetClientRoleState<ClientRoleState> for Coward {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}

impl Coward {
	pub fn won(game: &Game, actor_ref: PlayerReference)->bool{
        actor_ref.alive(game)
    }
}