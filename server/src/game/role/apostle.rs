use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::components::cult::{Cult, CultAbility};
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;
use crate::game::game_conclusion::GameConclusion;

use crate::game::role_list::RoleSet;
use crate::game::visit::Visit;
use crate::game::win_condition::WinCondition;
use crate::game::Game;
use super::{common_role, ControllerID, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Apostle;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Apostle {
    type ClientRoleState = Apostle;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        match (priority, Cult::next_ability(game)) {
            (Priority::Kill, CultAbility::Kill) if game.cult().ordered_nice_listers.len() == 1 => {

                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                let target_ref = visit.target;
                
                if target_ref.try_night_kill_single_attacker(
                    actor_ref, game, GraveKiller::RoleSet(RoleSet::Cult), AttackPower::Basic, false
                ) {
                    Cult::set_ability_used_last_night(game, Some(CultAbility::Kill));
                }
            }
            (Priority::Convert, CultAbility::Convert) => {
                
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                let target_ref = visit.target;

                if target_ref.night_defense(game).can_block(AttackPower::Basic) {
                    actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                    return
                }

                target_ref.set_night_convert_role_to(game, Some(Role::Zealot.new_state(game)));
                InsiderGroupID::Cult.add_player_to_revealed_group(game, target_ref);
                target_ref.set_win_condition(game, WinCondition::new_loyalist(GameConclusion::Cult));
                
                Cult::set_ability_used_last_night(game, Some(CultAbility::Convert));
            }
            _ => {}
        }
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game,
            actor_ref, 
            ControllerID::role(actor_ref, Role::Apostle, 0),
            Cult::next_ability(game) == CultAbility::Kill
        )
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        let grayed_out =
            game.cult().ordered_nice_listers.len() != 1 &&
            Cult::next_ability(game) == CultAbility::Kill;
        
        common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            grayed_out,
            ControllerID::role(actor_ref, Role::Apostle, 0)
        )
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Cult
        ].into_iter().collect()
    }
}
