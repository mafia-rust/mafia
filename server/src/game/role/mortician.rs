
use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::detained::Detained;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::grave::GraveInformation;
use crate::game::grave::GraveReference;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set;
use crate::vec_set::VecSet;
use super::AbilitySelection;
use super::ControllerID;
use super::ControllerParametersMap;
use super::Role;
use super::{Priority, RoleState, RoleStateImpl};


#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mortician {
    obscured_players: VecSet<PlayerReference>,
    cremations_remaining: u8,
}
const MAX_CREMATIONS: u8 = 3;
impl Default for Mortician {
    fn default() -> Self {
        Self {
            obscured_players: VecSet::new(),
            cremations_remaining: MAX_CREMATIONS,
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;


impl RoleStateImpl for Mortician {
    type ClientRoleState = Mortician;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Deception=>{
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else{return};

                let target_ref = visit.target;
                
                if !self.obscured_players.contains(&target_ref){
                    self.obscured_players.insert(target_ref);
                    actor_ref.set_role_state(game, RoleState::Mortician(self));

                    for player in InsiderGroupID::all_players_in_same_revealed_group_with_actor(game, actor_ref){
                        player.push_player_tag(game, target_ref, Tag::MorticianTagged);
                    }
                }
            },
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        let grayed_out = 
            !actor_ref.alive(game) || 
            Detained::is_detained(game, actor_ref);

        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Mortician, 0),
            super::AvailableAbilitySelection::new_player_list(
                PlayerReference::all_players(game)
                    .into_iter()
                    .filter(|p| *p != actor_ref)
                    .filter(|player| 
                        player.alive(game) &&
                        !self.obscured_players.contains(&player)
                    )
                    .collect(),
                false,
                Some(1)
            ),
            AbilitySelection::new_player_list(vec![]),
            grayed_out,
            Some(PhaseType::Obituary),
            false,
            vec_set![actor_ref]
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Mortician, 0),
            false
        )
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, _old: RoleState, new: RoleState){
        if player == actor_ref && new.role() != Role::Mortician {
            actor_ref.remove_player_tag_on_all(game, Tag::MorticianTagged);
        }
    }
    fn on_grave_added(mut self, game: &mut Game, actor_ref: PlayerReference, grave_ref: GraveReference){
        if
            actor_ref.alive(game) &&
            self.obscured_players.contains(&grave_ref.deref(game).player) &&
            self.cremations_remaining > 0
        {
            actor_ref.add_private_chat_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
                player: grave_ref.deref(game).player,
                role: grave_ref.deref(game).player.role(game),
                will: grave_ref.deref(game).player.will(game).to_string(),
            });
            self.cremations_remaining = self.cremations_remaining.saturating_sub(1);

            grave_ref.deref_mut(game).information = GraveInformation::Obscured;
            
            actor_ref.set_role_state(game, self);
        }
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}
