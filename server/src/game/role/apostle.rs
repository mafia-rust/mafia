use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::ChatMessageVariant;
use crate::game::components::cult::{Cult, CultAbility};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;
use crate::game::resolution_state::ResolutionState;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::win_condition::WinCondition;
use crate::game::Game;
use super::zealot::Zealot;
use super::{Priority, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Apostle{
    night_selection: <Apostle as RoleStateImpl>::RoleActionChoice,
}

pub(super) const FACTION: Faction = Faction::Cult;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Apostle {
    type ClientRoleState = Apostle;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        match (priority, Cult::next_ability(game)) {
            (Priority::Kill, CultAbility::Kill) if game.cult().ordered_cultists.len() == 1 => {

                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;
                
                if target_ref.try_night_kill_single_attacker(
                    actor_ref, game, GraveKiller::Faction(Faction::Cult), AttackPower::Basic, false
                ) {
                    Cult::set_ability_used_last_night(game, Some(CultAbility::Kill));
                }
            }
            (Priority::Convert, CultAbility::Convert) => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;

                if target_ref.night_defense(game).can_block(AttackPower::Basic) {
                    actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                    return
                }

                target_ref.set_role(game, RoleState::Zealot(Zealot));
                target_ref.set_win_condition(game, WinCondition::new_single_resolution_state(ResolutionState::Cult));
                Cult::set_ability_used_last_night(game, Some(CultAbility::Convert));
            }
            _ => {}
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, &action_choice){
            return
        }

        let cult = game.cult().clone();
        let can_kill = cult.ordered_cultists.len() == 1 && Cult::next_ability(game) == CultAbility::Kill;
        let can_convert = Cult::next_ability(game) == CultAbility::Convert;

        if !can_convert && !can_kill {return}

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self.into());
    }
    fn create_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(game,
            actor_ref,
            &self.night_selection,
            Cult::next_ability(game) == CultAbility::Kill
        )
    }
}
