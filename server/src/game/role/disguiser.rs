use std::iter::once;

use serde::Serialize;

use crate::game::ability_input::ability_selection::AvailableAbilitySelection;
use crate::game::ability_input::selection_type::one_player_option_selection::AvailableOnePlayerOptionSelection;
use crate::game::ability_input::selection_type::role_option_selection::AvailableRoleOptionSelection;
use crate::game::ability_input::{AbilityID, AbilityInput, AvailableAbilityInput};
use crate::game::chat::ChatMessageVariant;
use crate::game::components::detained::Detained;
use crate::game::grave::GraveInformation;
use crate::game::phase::PhaseType;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::Visit;

use crate::game::Game;
use super::{InsiderGroupID, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Disguiser{
    pub current_target: Option<PlayerReference>,
    pub disguised_role: Role,
}
impl Default for Disguiser{
    fn default() -> Self {
        Self{
            current_target: None,
            disguised_role: Role::Jester,
        }
    }
}

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Disguiser {
    type ClientRoleState = Disguiser;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Deception => {
                
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(first_visit) = actor_visits.first() else {return};
                
                actor_ref.remove_player_tag_on_all(game, crate::game::tag::Tag::Disguise);
                self.current_target = Some(first_visit.target);
                actor_ref.push_player_tag(game, first_visit.target, crate::game::tag::Tag::Disguise);

                actor_ref.set_role_state(game, self);
            },
            _ => {}
        }
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_saved_ability_to_visits(game, actor_ref, AbilityID::role(Role::Disguiser, 0), false)
    }
    fn available_ability_input(self, game: &Game, actor_ref: PlayerReference) -> crate::game::ability_input::AvailableAbilityInput {
        if !actor_ref.alive(game) {return AvailableAbilityInput::default()};

        let mut out = if game.current_phase().phase() == PhaseType::Night {
            AvailableAbilityInput::new_ability(
                AbilityID::Role { role: Role::Disguiser, id: 0 }, 
                AvailableAbilitySelection::OnePlayerOption {
                    selection: AvailableOnePlayerOptionSelection(
                        PlayerReference::all_players(game)
                            .filter(|p|
                                !Detained::is_detained(game, actor_ref) &&
                                actor_ref.alive(game) &&
                                p.alive(game) &&
                                InsiderGroupID::in_same_revealed_group(game, actor_ref, *p)
                            )
                            .map(|p| Some(p))
                            .chain(once(None))
                            .collect()
                    )
                }
            )
        }else{
            AvailableAbilityInput::default()
        };
        
        out.combine_overwrite(
            AvailableAbilityInput::new_ability(
                AbilityID::Role { role: Role::Disguiser, id: 1 },
                AvailableAbilitySelection::RoleOption {
                    selection: AvailableRoleOptionSelection(
                        Role::values().into_iter()
                            .map(|role| Some(role))
                            .collect()
                    )
                }
            )
        );
        out
    }
    fn on_ability_input_received(mut self, game: &mut Game, actor_ref: PlayerReference, input_player: PlayerReference, ability_input: AbilityInput) {
        if actor_ref != input_player {return;}

        if let Some(selection) = ability_input.get_role_option_selection_if_id(
            AbilityID::role(Role::Disguiser, 1)
        ) {
            if let Some(target) = selection.0 {
                self.disguised_role = target;
            }
        };
        
        actor_ref.set_role_state(game, self);
    }
    fn on_any_death(mut self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference) {
        if
            self.current_target.is_some_and(|p|p == dead_player_ref) || 
            actor_ref == dead_player_ref
        {
            actor_ref.remove_player_tag_on_all(game, crate::game::tag::Tag::Disguise);
            self.current_target = None;
            actor_ref.set_role_state(game, self);
        }
    }
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave: crate::game::grave::GraveReference) {
        let grave_ref = grave;
        
        if
            self.current_target.is_some_and(|p|p == grave.deref(game).player) && (
                actor_ref.alive(game) ||
                self.current_target.is_some_and(|p|p == actor_ref)
            )
        {
            actor_ref.add_private_chat_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
                player: grave.deref(game).player,
                role: grave.deref(game).player.role(game),
                will: grave.deref(game).player.will(game).to_string(),
            });
            
            let mut grave = grave_ref.deref(game).clone();
            *grave_ref.deref_mut(game) = match grave.information {
                GraveInformation::Normal{role: _, will, death_cause, death_notes} => {
                    grave.information = GraveInformation::Normal{
                        role: self.disguised_role,
                        will,
                        death_cause,
                        death_notes
                    };
                    grave
                },
                _ => grave
            };
        }
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}


/*

            Priority::Investigative => {
                if actor_ref.alive(game) || actor_ref.night_blocked(game) {return;}

                let mut chat_messages = Vec::new();

                for player in PlayerReference::all_players(game){
                    if !InsiderGroupID::in_same_revealed_group(game, actor_ref, player) {continue;}

                    let visitors_roles: Vec<Role> = PlayerReference::all_appeared_visitors(player, game)
                        .iter()
                        .filter(|player|
                            player.win_condition(game)
                                .is_loyalist_for(crate::game::game_conclusion::GameConclusion::Town)
                        )
                        .map(|player| player.role(game))
                        .collect();


                    chat_messages.push(ChatMessageVariant::FramerResult{mafia_member: player.index(), visitors: visitors_roles});
                }

                for player in PlayerReference::all_players(game){
                    if !InsiderGroupID::in_same_revealed_group(game, actor_ref, player) {continue;}
                    for msg in chat_messages.iter(){
                        player.push_night_message(game, msg.clone());
                    }
                }
            },
 */