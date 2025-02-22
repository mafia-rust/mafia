use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set;
use super::{Priority, Role, RoleState, RoleStateImpl};
use crate::game::ability_input::*;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Yer{
    star_passes_remaining: u8,
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
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() == 1 {return}

        let chose_to_convert = if let Some(BooleanSelection(bool)) = game.saved_controllers.get_controller_current_selection_boolean(
            ControllerID::role(actor_ref, Role::Yer, 0)
        ){
            bool
        }else{false};

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first(){
            let target_ref = visit.target;

            if !chose_to_convert{
                if priority != Priority::Kill {return}

                target_ref.try_night_kill_single_attacker(
                    actor_ref,
                    game,
                    GraveKiller::Role(Role::Yer),
                    AttackPower::ArmorPiercing,
                    true
                );
            }else{
                if priority != Priority::Convert {return}
                if self.star_passes_remaining <= 0 {return}

                if target_ref.night_defense(game).can_block(AttackPower::ArmorPiercing) {
                    actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                    return
                }

                self.star_passes_remaining = self.star_passes_remaining.saturating_sub(1);

                //role switching stuff
                let fake_role = if let Some(RoleOptionSelection(Some(role))) = game.saved_controllers.get_controller_current_selection_role_option(
                    ControllerID::role(actor_ref, Role::Yer, 2)
                ){
                    role
                }else{Role::Yer};

                actor_ref.set_night_grave_role(game, Some(fake_role));
                
                //convert & kill stuff
                target_ref.set_win_condition(
                    game,
                    WinCondition::new_loyalist(crate::game::game_conclusion::GameConclusion::Fiends)
                );
                target_ref.set_night_convert_role_to(game, Some(RoleState::Yer(self.clone())));

                actor_ref.try_night_kill_single_attacker(
                    actor_ref,
                    game,
                    GraveKiller::Role(Role::Yer),
                    AttackPower::ProtectionPiercing,
                    true
                );

                actor_ref.set_role_state(game, self);
            }
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_boolean(
            game,
            actor_ref,
            self.star_passes_remaining <= 0 || game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Yer, 0)
        ).combine_overwrite_owned(
            crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
                game,
                actor_ref,
                false,
                true,
                game.day_number() <= 1,
                ControllerID::role(actor_ref, Role::Yer, 1)
            )
        ).combine_overwrite_owned(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Yer, 2),
                AvailableAbilitySelection::new_role_option(
                    Role::values().into_iter()
                        .map(|role| Some(role))
                        .collect()
                ),
                AbilitySelection::new_role_option(Some(Role::Yer)),
                self.star_passes_remaining <= 0 ||
                actor_ref.ability_deactivated_from_death(game) ||
                game.day_number() <= 1,
                None,
                false,
                vec_set!(actor_ref)
            )
        )
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
}