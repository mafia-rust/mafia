use serde::Serialize;

use crate::game::components::tags::{TagSetID, Tags};
use crate::game::ability_input::AvailablePlayerListSelection;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::role_list::RoleSet;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::{Visit, VisitTag};

use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer;

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Framer {
    type ClientRoleState = Framer;
    fn on_midnight(self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        match priority {
            OnMidnightPriority::Deception => {
                let framer_visits = actor_ref.untagged_night_visits_cloned(game).clone();

                let Some(first_visit) = framer_visits.first() else {return};

                Tags::add_tag(game, TagSetID::Framer(actor_ref), first_visit.target);

                for framed_target in Tags::tagged(game, TagSetID::Framer(actor_ref)){
                    framed_target.set_night_framed(midnight_variables, true);
                }

                let Some(second_visit) = framer_visits.get(1) else {return};
            
                first_visit.target.set_night_appeared_visits(midnight_variables, Some(vec![
                    Visit::new_role(first_visit.target, second_visit.target, false, first_visit.target.role(game), 0)
                ]));

                actor_ref.set_night_visits(
                    game,
                    actor_ref.all_night_visits_cloned(game)
                        .into_iter()
                        .filter(|v|v.tag!=VisitTag::Role { role: Role::Framer, id: 1 })
                        .collect::<Vec<_>>()
                );
            },
            OnMidnightPriority::Investigative => {
                Tags::set_tagged(
                    game,
                    TagSetID::Framer(actor_ref),
                    &Tags::tagged(game, TagSetID::Framer(actor_ref))
                        .into_iter()
                        .filter(|p|
                            !p.all_night_visitors_cloned(game).iter().any(|visitor| {
                                RoleSet::TownInvestigative.get_roles().contains(&visitor.role(game))
                            })
                        )
                        .collect()
                );
            },
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::combine([
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Framer, 0))
                .single_player_selection_typical(actor_ref, false, false)
                .night_typical(actor_ref)
                .build_map(),
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Framer, 1))
                .available_selection(AvailablePlayerListSelection {
                    available_players: PlayerReference::all_players(game).collect(),
                    can_choose_duplicates: false,
                    max_players: Some(1)
                })
                .night_typical(actor_ref)
                .add_grayed_out_condition(
                    // Framed player is not selected
                    ControllerID::role(actor_ref, Role::Framer, 0)
                        .get_player_list_selection(game)
                        .is_none_or(|selection| selection.0.is_empty())
                )
                .build_map()
        ])
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Framer, 0),
            false,
        ).into_iter().chain(
            crate::game::role::common_role::convert_controller_selection_to_visits_visit_tag(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Framer, 1),
                false,
                VisitTag::Role { role: Role::Framer, id: 1 }
            )
        ).collect()
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }

    
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        Tags::add_viewer(game, TagSetID::Framer(actor_ref), actor_ref);
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, _new: super::RoleState, _old: super::RoleState) {
        if actor_ref != player {return}
        Tags::remove_viewer(game, TagSetID::Framer(actor_ref), actor_ref);
    }
}