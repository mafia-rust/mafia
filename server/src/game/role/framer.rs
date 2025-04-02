use serde::Serialize;

use crate::game::ability_input::AvailablePlayerListSelection;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::role_list::RoleSet;
use crate::game::tag::Tag;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::{Visit, VisitTag};

use crate::game::Game;
use crate::vec_set::VecSet;
use super::{ControllerID, ControllerParametersMap, Role, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer{
    framed_targets: VecSet<PlayerReference>
}

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Framer {
    type ClientRoleState = Framer;
    fn on_midnight(mut self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        match priority {
            OnMidnightPriority::Deception => {
                let framer_visits = actor_ref.untagged_night_visits_cloned(game).clone();

                let Some(first_visit) = framer_visits.first() else {return};

                self.framed_targets.insert(first_visit.target);

                first_visit.target.set_night_framed(midnight_variables, true);
                for framed_target in self.framed_targets.iter(){
                    framed_target.set_night_framed(midnight_variables, true);
                }
                self.update_framer_tags(game, actor_ref);
                actor_ref.set_role_state(game, self);

                let Some(second_visit) = framer_visits.get(1) else {return};
            
                first_visit.target.set_night_appeared_visits(midnight_variables, Some(vec![
                    Visit::new_none(first_visit.target, second_visit.target, false)
                ]));

                //this code erases only the second framer visit
                let mut new_visits = vec![];
                let mut got_first = false;
                for visit in actor_ref.all_night_visits_cloned(game){
                    if visit.tag == VisitTag::Role {
                        if !got_first {
                            new_visits.push(visit);
                        }
                        got_first = true;
                    }else{
                        new_visits.push(visit);
                    }
                }
                actor_ref.set_night_visits(game, new_visits);
            }
            OnMidnightPriority::Investigative => {
                self.framed_targets.retain(|p|
                    !p.all_appeared_visitors(game, midnight_variables).iter().any(|visitor| {
                        RoleSet::TownInvestigative.get_roles().contains(&visitor.role(game))
                    })
                );

                self.update_framer_tags(game, actor_ref);
                actor_ref.set_role_state(game, self);
            }
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
                    game.saved_controllers
                        .get_controller_current_selection_player_list(ControllerID::role(actor_ref, Role::Framer, 0))
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
            false
        ).into_iter().chain(
            crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Framer, 1),
                false
            )
        ).collect()
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}
impl Framer {
    pub fn update_framer_tags(&self, game: &mut Game, actor_ref: PlayerReference){
        for player in PlayerReference::all_players(game){
            match (
                actor_ref.player_has_tag(game, player, Tag::Frame) != 0, 
                self.framed_targets.contains(&player)
            ){
                (false, true) => {
                    actor_ref.push_player_tag(game, player, Tag::Frame);
                }
                (true, false) => {
                    actor_ref.remove_player_tag(game, player, Tag::Frame);
                }
                _ => {}
            }
        }
    }
}