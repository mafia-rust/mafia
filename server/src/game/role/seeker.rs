use std::collections::HashSet;

use serde::Serialize;

use crate::game::ability_input::AvailablePlayerListSelection;
use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::event::on_midnight::MidnightVariables;
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set::VecSet;
use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Seeker {
    followers: HashSet<PlayerReference>,
    new_follower: Option<PlayerReference>,
}

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Seeker {
    type ClientRoleState = Seeker;
    fn on_midnight(mut self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        for follower in self.followers.iter() {
            actor_ref.possess_night_action(game, midnight_variables, priority, Some(*follower));
        }
        match (priority, &self.new_follower) {
            (OnMidnightPriority::Investigative, _) => {
                let Some(camp) = actor_ref.untagged_night_visits(midnight_variables).first().map(|v|v.target) else {return};
                let Some(&mark) = ControllerID::role(actor_ref, Role::Seeker, 0)
                    .get_player_list_selection(game)
                    .cloned()
                    .unwrap_or_default()
                    .0.first()
                        else {return};
                if camp.all_night_visitors_cloned(midnight_variables).iter().any(|p|*p==mark) {
                    self.new_follower = Some(mark);
                    actor_ref.set_role_state(game, self);
                    mark.push_night_message(midnight_variables, ChatMessageVariant::CaughtBySeeker);
                    actor_ref.push_night_message(midnight_variables, ChatMessageVariant::SeekerCaught{player: mark});
                }
            }
            (OnMidnightPriority::FinalizeNight, Some(follower)) => {
                self.followers.insert(*follower);
                self.new_follower = None;
                if self.won() {
                    actor_ref.leave_town(game);
                }
                actor_ref.set_role_state(game, self);
            }
            _=>(),
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::combine([
            // Mark
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Seeker, 0))
                .available_selection(AvailablePlayerListSelection {
                    available_players: PlayerReference::all_players(game)
                        .filter(|p|
                            p.alive(game) && 
                            *p != actor_ref
                        )
                        .collect::<VecSet<_>>(),
                    can_choose_duplicates: false,
                    max_players: Some(1)
                })
                .night_typical(actor_ref)
                .add_grayed_out_condition(self.won())
                .build_map(),
            // Camp
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Seeker, 1))
                .available_selection(AvailablePlayerListSelection {
                    available_players: PlayerReference::all_players(game)
                        .filter(|p|
                            p.alive(game) && 
                            *p != actor_ref
                        )
                        .collect::<VecSet<_>>(),
                    can_choose_duplicates: false,
                    max_players: Some(1)
                })
                .night_typical(actor_ref)
                .add_grayed_out_condition(self.won())
                .build_map()
        ])
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Seeker, 1),
            false
        )
    }
}

impl Seeker {
    pub fn won(&self) -> bool {
        self.followers.len() >= 3 ||
        !self.new_follower.is_some_and(|f|
            self.followers.len() == 2 && !self.followers.contains(&f)
        )
    }
}