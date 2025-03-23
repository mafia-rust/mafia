use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::grave::{GraveInformation, GraveReference};
use crate::game::player::PlayerReference;

use super::{
	ControllerID, ControllerParametersMap, GetClientRoleState, Priority, Role, RoleStateImpl,
};
use crate::game::visit::Visit;
use crate::game::Game;

#[derive(Clone, Debug, Default)]
pub struct Coward {
	obscure: Option<PlayerReference>,
	saved: bool,
}

#[derive(Clone, Serialize, Debug)]
pub struct ClientRoleState;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Coward {
	type ClientRoleState = ClientRoleState;

	fn redirect_attack(
		self,
		game: &mut Game,
		actor_ref: PlayerReference,
		attack: AttackPower,
		with_visit: bool,
	) -> Option<(PlayerReference, AttackPower)> {
		if with_visit {
			if let Some(target) = self.obscure {
				actor_ref.set_role_state(
					game,
					Coward {
						obscure: self.obscure,
						saved: true,
					},
				);
				return Some((target, AttackPower::ProtectionPiercing));
			}
		}
		return Some((actor_ref, attack));
	}

	fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
		match priority {
			Priority::Heal => {
				let obscure = match actor_ref.untagged_night_visits_cloned(game).first() {
					None => None,
					Some(target) => Some(target.target),
				};
				if let Some(_) = obscure {
					actor_ref.push_night_message(game, ChatMessageVariant::CowardHid);
				}
				actor_ref.set_role_state(
					game,
					Coward {
						obscure,
						saved: false,
					},
				)
			}
			// using SpyBug priority because it seems like it won't cause any issues/nothing else goes here.
			Priority::SpyBug => {
				if self.saved {
					actor_ref.push_night_message(game, ChatMessageVariant::CowardSaved);
				}
			}
			_ => {}
		}
	}

	fn controller_parameters_map(
		self,
		game: &Game,
		actor_ref: PlayerReference,
	) -> ControllerParametersMap {
		crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
			game,
			actor_ref,
			false,
			true,
			false,
			ControllerID::role(actor_ref, Role::Coward, 0),
		)
	}
	fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
		crate::game::role::common_role::convert_controller_selection_to_visits(
			game,
			actor_ref,
			ControllerID::role(actor_ref, Role::Coward, 0),
			true,
		)
	}

	fn on_grave_added(self, game: &mut Game, _: PlayerReference, grave_ref: GraveReference) {
		if let Some(player) = self.obscure {
			if player == grave_ref.deref(game).player {
				grave_ref.deref_mut(game).information = GraveInformation::Obscured;
			}
		}
	}
	fn on_phase_start(
		self,
		_game: &mut Game,
		_actor_ref: PlayerReference,
		_phase: crate::game::phase::PhaseType,
	) {
		if let crate::game::phase::PhaseType::Night = _phase {
			let obscure = None;
			_actor_ref.set_role_state(
				_game,
				Coward {
					obscure,
					saved: false,
				},
			);
		}
	}
}

impl GetClientRoleState<ClientRoleState> for Coward {
	fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
		ClientRoleState
	}
}

impl Coward {
	pub fn won(game: &Game, actor_ref: PlayerReference) -> bool {
		actor_ref.alive(game)
	}
}
