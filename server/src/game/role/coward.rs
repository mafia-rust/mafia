
use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::components::night_visits::NightVisits;
use crate::game::grave::{GraveInformation, GraveKiller, GraveReference};
use crate::game::chat::ChatMessageVariant;
use crate::game::player::PlayerReference;


use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set::VecSet;
use super::{
	ControllerID, ControllerParametersMap, GetClientRoleState, Priority, Role, RoleStateImpl
};

#[derive(Clone, Debug, Default)]
pub struct Coward {
	pub tagged_for_obscure: VecSet<PlayerReference>
}

#[derive(Clone, Serialize, Debug)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Coward {
    type ClientRoleState = ClientRoleState;

    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
    	let actor_visits = actor_ref.untagged_night_visits_cloned(game);
    
	   	match priority {
	        Priority::Kill => {
	
	        if let Some(target) = actor_visits.first(){
		       	actor_ref.push_night_message(game, 
		            ChatMessageVariant::CowardHid
		        );
	        
				let mut tagged_for_obscure = self.tagged_for_obscure.clone();
				
				let mut saved : bool = false;
				let mut attack_success : bool = false;
				for visit in NightVisits::all_visits(game).into_iter().copied().collect::<Vec<_>>() {
	                if 
	                    visit.attack &&
	                    visit.target == actor_ref &&
	                    visit.visitor != actor_ref
	                {
	                	saved = true;
	                	attack_success |= target.visitor.try_night_kill_single_attacker(
	                 		actor_ref, game, GraveKiller::Role(Role::Engineer), AttackPower::ProtectionPiercing, false
	                 	);
	                }
	            }
	            if attack_success {
		            tagged_for_obscure.insert(target.visitor);
		        }
	            if saved {
		           	actor_ref.push_night_message(game, 
			            ChatMessageVariant::CowardSaved
			        );
	            }
				
	            actor_ref.set_role_state(game, Coward{tagged_for_obscure});
	        }
	
	        }
	        Priority::Heal => {
	            if let Some(_) = actor_visits.first(){
	            	actor_ref.increase_defense_to(game, DefensePower::Protection);
	            }
	        }
	        _ => {}
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
        if !self.tagged_for_obscure.contains(&grave_ref.deref(game).player) {return}

        grave_ref.deref_mut(game).information = GraveInformation::Obscured;
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