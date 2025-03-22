
use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::attack_type::AttackData;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::Game;
use crate::vec_set;
use super::{AbilitySelection, ControllerID, ControllerParametersMap, PlayerListSelection, Role, RoleStateImpl};




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


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Deputy {
    type ClientRoleState = Deputy;
    fn on_validated_ability_input_received(self, game: &mut Game, actor_ref: PlayerReference, input_player: PlayerReference, ability_input: super::AbilityInput) {
        
        if actor_ref != input_player {return;}
        let Some(PlayerListSelection(target_ref)) = ability_input.get_player_list_selection_if_id(
            ControllerID::role(actor_ref, Role::Deputy, 0)
        )else{return};
        let Some(target_ref) = target_ref.first() else {return};
        
        
        target_ref.add_private_chat_message(game, ChatMessageVariant::DeputyShotYou);
        if target_ref.defense(game).can_block(AttackPower::Basic) {
            target_ref.add_private_chat_message(game, ChatMessageVariant::YouSurvivedAttack);
            actor_ref.add_private_chat_message(game, ChatMessageVariant::SomeoneSurvivedYourAttack);

        }else{
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::DeputyKilled{shot_index: target_ref.index()});
            
            
            let mut grave = Grave::from_player_lynch(game, *target_ref);
            if let GraveInformation::Normal{death_cause, ..} = &mut grave.information {
                *death_cause = GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Deputy)]);
            }
            target_ref.die(game, grave);
            

            if target_ref.win_condition(game).is_loyalist_for(GameConclusion::Town) {
                actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
            }
        }

        actor_ref.set_role_state(game, Deputy{bullets_remaining:self.bullets_remaining.saturating_sub(1)});
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Deputy, 0),
            super::AvailableAbilitySelection::new_player_list(
                PlayerReference::all_players(game)
                    .filter(|player| 
                        actor_ref != *player &&
                        player.alive(game)
                    )
                    .collect(),
                false,
                Some(1)
            ),
            AbilitySelection::new_player_list(vec![]),
            actor_ref.ability_deactivated_from_death(game) ||
            self.bullets_remaining == 0 || 
            game.day_number() <= 1 || 
            !(PhaseType::Discussion == game.current_phase().phase() || PhaseType::Nomination == game.current_phase().phase()),
            None,
            true,
            vec_set!(actor_ref)
        )
    }
    fn attack_data(&self, game: &Game, actor_ref: PlayerReference) -> AttackData {
        if self.bullets_remaining > 0 && actor_ref.alive(game) {
            AttackData::attack(game, actor_ref, true)
        } else {
            AttackData::none()
        }
    }
}