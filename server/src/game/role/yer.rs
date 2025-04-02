use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set::VecSet;
use super::{Role, RoleState, RoleStateImpl};
use crate::game::ability_input::*;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Yer{
    pub star_passes_remaining: u8,
    pub old_role: Role,
}

impl Default for Yer {
    fn default() -> Yer {
        Yer {
            star_passes_remaining: 3,
            old_role: Role::Yer
        }
    }
}

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Yer {
    type ClientRoleState = Yer;
    fn new_state(game: &Game) -> Self {
        Self{
            star_passes_remaining: game.num_players().div_ceil(5),
            ..Self::default()
        }
    }
    fn on_midnight(mut self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        if game.day_number() == 1 {return}

        let chose_to_convert = game.saved_controllers.get_controller_current_selection_boolean(
            ControllerID::role(actor_ref, Role::Yer, 0)
        ).map(|selection| selection.0).unwrap_or(false);

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            let target_ref = visit.target;

            if !chose_to_convert {
                if priority != OnMidnightPriority::Kill {return}

                target_ref.try_night_kill_single_attacker(
                    actor_ref,
                    game,
                    GraveKiller::Role(Role::Yer),
                    AttackPower::ArmorPiercing,
                    true,
                    true
                );
            } else {
                if priority != OnMidnightPriority::Convert {return}
                if self.star_passes_remaining == 0 {return}

                if let Some(target) = target_ref.try_convert(
                	actor_ref, game, AttackPower::ArmorPiercing, true,
                 	RoleState::Yer(Yer { 
						star_passes_remaining: self.star_passes_remaining.saturating_sub(1), 
						old_role: target_ref.role(game),
					})
                ).successful_target() {
	               	self.star_passes_remaining = self.star_passes_remaining.saturating_sub(1);
	
	                //role switching stuff
	                let fake_role = self.current_fake_role(game, actor_ref);
	
	                actor_ref.set_night_grave_role(game, Some(fake_role));
	                
	                //convert & kill stuff
	                target.set_win_condition(
	                    game,
	                    WinCondition::new_loyalist(crate::game::game_conclusion::GameConclusion::Fiends)
	                );
	
	                actor_ref.try_night_kill_single_attacker(
	                    actor_ref,
	                    game,
	                    GraveKiller::Role(Role::Yer),
	                    AttackPower::ProtectionPiercing,
	                    true,
	                    false
	                );
	
	                actor_ref.set_role_state(game, self);
                } else {
                	actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                }
            }
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::combine([
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Yer, 0))
                .available_selection(AvailableBooleanSelection)
                .add_grayed_out_condition(self.star_passes_remaining == 0 || game.day_number() <= 1)
                .night_typical(actor_ref)
                .build_map(),
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Yer, 1))
                .single_player_selection_typical(actor_ref, false, true)
                .night_typical(actor_ref)
                .add_grayed_out_condition(game.day_number() <= 1)
                .build_map(),
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Yer, 2))
                .available_selection(AvailableRoleOptionSelection(
                    game.settings.enabled_roles.iter()
                        .map(|role| Some(*role))
                        .collect::<VecSet<Option<Role>>>()
                ))
                .default_selection(RoleOptionSelection(Some(self.old_role)))
                .add_grayed_out_condition(
                    self.star_passes_remaining == 0 ||
                    actor_ref.ability_deactivated_from_death(game) ||
                    game.day_number() <= 1
                )
                .allow_players([actor_ref])
                .build_map()
        ])
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Yer, 1),
            true
        )
    }
}


impl Yer{
    pub fn current_fake_role(&self, game: &Game, actor_ref: PlayerReference) -> Role {
        if let Some(RoleOptionSelection(Some(role))) = game.saved_controllers.get_controller_current_selection_role_option(
            ControllerID::role(actor_ref, Role::Yer, 2)
        ){
            role
        } else {
            self.old_role
        }
    }
}