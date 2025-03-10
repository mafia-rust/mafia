use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::attack_type::AttackData;
use crate::game::chat::ChatMessageVariant;
use crate::game::grave::{GraveInformation, GraveKiller, GraveReference};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set::VecSet;

use super::{ControllerID, ControllerParametersMap, GetClientRoleState, Priority, Role, RoleStateImpl};

#[derive(Debug, Clone, Default)]
pub struct Pyrolisk{
    pub tagged_for_obscure: VecSet<PlayerReference>
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Pyrolisk {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() <= 1 {return;}

        match priority {
            Priority::Kill => {
                let mut tagged_for_obscure = self.tagged_for_obscure.clone();

                let mut killed_at_least_once = false;

                for other_player_ref in actor_ref.all_night_visitors_cloned(game)
                    .into_iter().filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref
                    ).collect::<Vec<PlayerReference>>()
                {
                    let attack_success = other_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Pyrolisk), AttackPower::ArmorPiercing, true);
                    if attack_success {
                        tagged_for_obscure.insert(other_player_ref);
                        killed_at_least_once = true;
                    }
                    
                }

                if !killed_at_least_once {
                    let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                    if let Some(visit) = actor_visits.first(){
                        let attack_success = visit.target.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Pyrolisk), AttackPower::ArmorPiercing, true);
                        if attack_success {
                            tagged_for_obscure.insert(visit.target);
                        }
                    }
                }
                
                actor_ref.set_role_state(game, Pyrolisk{tagged_for_obscure});
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
            game.day_number() <= 1 ,
            ControllerID::role(actor_ref, Role::Pyrolisk, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Pyrolisk, 0),
            true
        )
    }
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave_ref: GraveReference){
        if !actor_ref.alive(game) && grave_ref.deref(game).player != actor_ref {return}
        if !self.tagged_for_obscure.contains(&grave_ref.deref(game).player) && grave_ref.deref(game).player != actor_ref {return}
         
        actor_ref.add_private_chat_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
            player: grave_ref.deref(game).player,
            role: grave_ref.deref(game).player.role(game),
            will: grave_ref.deref(game).player.will(game).to_string(),
        });

        grave_ref.deref_mut(game).information = GraveInformation::Obscured;
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: crate::game::phase::PhaseType) {
        actor_ref.set_role_state(game, Pyrolisk{tagged_for_obscure: VecSet::new()});
    }
    fn attack_data(&self, game: &Game, actor_ref: PlayerReference) -> AttackData {
        AttackData::attack(game, actor_ref, false, false)
    }
}
impl GetClientRoleState<ClientRoleState> for Pyrolisk {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}