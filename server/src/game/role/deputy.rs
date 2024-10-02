
use serde::{Deserialize, Serialize};

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::resolution_state::ResolutionState;
use crate::game::grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::Game;
use super::{RoleStateImpl, Role, RoleState};




#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Deputy {
    bullets_remaining: u8,
}

impl Default for Deputy {
    fn default() -> Self {
        Self { bullets_remaining: 1 }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    player: PlayerReference,
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Deputy {
    type ClientRoleState = Self;
    type RoleActionChoice = RoleActionChoice;
    fn on_role_action(self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if !(
            game.current_phase().is_day() &&
            game.phase_machine.day_number > 1 &&
            self.bullets_remaining > 0 &&
            actor_ref != action_choice.player &&
            action_choice.player.alive(game) && actor_ref.alive(game) &&
            (PhaseType::Discussion == game.current_phase().phase() || PhaseType::Nomination == game.current_phase().phase())
        ){
            return;
        }


        action_choice.player.add_private_chat_message(game, ChatMessageVariant::DeputyShotYou);
        if action_choice.player.defense(game).can_block(AttackPower::Basic) {
            action_choice.player.add_private_chat_message(game, ChatMessageVariant::YouSurvivedAttack);
            actor_ref.add_private_chat_message(game, ChatMessageVariant::SomeoneSurvivedYourAttack);

        }else{
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::DeputyKilled{shot_index: action_choice.player.index()});
            
            
            let mut grave = Grave::from_player_lynch(game, action_choice.player);
            if let GraveInformation::Normal{death_cause, ..} = &mut grave.information {
                *death_cause = GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Deputy)]);
            }
            action_choice.player.die(game, grave);
            

            if action_choice.player.win_condition(game).requires_only_this_resolution_state(ResolutionState::Town) {
                actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
            }
        }

        actor_ref.set_role_state(game, RoleState::Deputy(Deputy{bullets_remaining:self.bullets_remaining-1}));
    }
}