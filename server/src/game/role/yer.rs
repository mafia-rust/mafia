use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set;
use super::{Priority, Role, RoleState, RoleStateImpl};
use crate::game::ability_input::*;

#[derive(Debug, Clone, Serialize, Default)]
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

                if target_ref.night_defense(game).can_block(AttackPower::Basic) {
                    actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                    return
                }
                
                self.star_passes_remaining = self.star_passes_remaining.saturating_sub(1);
                
                InsiderGroupID::Puppeteer.add_player_to_revealed_group(game, target_ref);
                target_ref.set_win_condition(
                    game,
                    WinCondition::new_loyalist(crate::game::game_conclusion::GameConclusion::Fiends)
                );
                target_ref.set_night_convert_role_to(game, Some(RoleState::Yer(self.clone())));

                actor_ref.set_role_state(game, self);
                actor_ref.try_night_kill_single_attacker(
                    actor_ref,
                    game,
                    GraveKiller::Role(Role::Yer),
                    AttackPower::ProtectionPiercing,
                    true
                );
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
                !actor_ref.alive(game),
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
    fn default_revealed_groups(self) -> vec_set::VecSet<InsiderGroupID> {
        vec_set![InsiderGroupID::Puppeteer]
    }
}


impl Yer{
}