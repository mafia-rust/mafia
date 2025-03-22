use serde::Serialize;

use crate::game::ability_input::*;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::detained::Detained;
use crate::game::grave::GraveInformation;
use crate::game::phase::PhaseType;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set::{vec_set, VecSet};
use super::{InsiderGroupID, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Disguiser{
    pub current_target: Option<PlayerReference>
}

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Disguiser {
    type ClientRoleState = Disguiser;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        if priority != Priority::Deception {return}
                
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        let Some(first_visit) = actor_visits.first() else {return};
        
        if !InsiderGroupID::in_same_revealed_group(game, actor_ref, first_visit.target) {return}

        actor_ref.remove_player_tag_on_all(game, crate::game::tag::Tag::Disguise);
        self.current_target = Some(first_visit.target);
        actor_ref.push_player_tag(game, first_visit.target, crate::game::tag::Tag::Disguise);

        actor_ref.set_role_state(game, self);
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Disguiser, 0),
            false
        )
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Disguiser, 0),
            AvailableAbilitySelection::new_player_list(PlayerReference::all_players(game)
                    .filter(|p|
                        p.alive(game) &&
                        InsiderGroupID::in_same_revealed_group(game, actor_ref, *p)
                    )
                    .collect(),
                    false,
                    Some(1)
                ),
            AbilitySelection::new_player_list(vec![]),
            actor_ref.ability_deactivated_from_death(game) ||
            Detained::is_detained(game, actor_ref),
            Some(PhaseType::Obituary),
            false,
            vec_set!(actor_ref)
        ).combine_overwrite_owned(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Disguiser, 1),
                AvailableAbilitySelection::new_role_option(
                    Role::values().into_iter()
                        .map(Some)
                        .collect()
                ),
                AbilitySelection::new_role_option(Some(Role::Disguiser)),
                actor_ref.ability_deactivated_from_death(game),
                None,
                false,
                self.players_with_disguiser_menu(actor_ref)
            )
        )
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
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Night => {
                actor_ref.remove_player_tag_on_all(game, crate::game::tag::Tag::Disguise);
                self.current_target = None;
                actor_ref.set_role_state(game, self);
            },
            _ => {}
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
                        role: Self::disguised_role(&self, game, actor_ref),
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
    fn attack_data(&self, _game: &Game, _actor_ref: PlayerReference) -> crate::game::attack_type::AttackData {
        crate::game::attack_type::AttackData::none()
    }
}

impl Disguiser{
    fn players_with_disguiser_menu(&self, actor_ref: PlayerReference)->VecSet<PlayerReference>{
        let mut players = vec_set!(actor_ref);
        if let Some(disguised) = self.current_target{
            players.insert(disguised);
        }
        players
    }
    fn disguised_role(&self, game: &Game, actor_ref: PlayerReference)->Role{
        if let Some(role) = game.saved_controllers.get_controller_current_selection_role_option(
            ControllerID::role(actor_ref, Role::Disguiser, 1)
        ).and_then(|selection| selection.0)
        {
            role
        }else{
            Role::Disguiser
        }
    }
}