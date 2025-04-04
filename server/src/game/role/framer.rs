use serde::Serialize;

use crate::game::components::detained::Detained;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::components::tags::{TagSetID, Tags};
use crate::game::role_list::RoleSet;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::{Visit, VisitTag};

use crate::game::Game;
use crate::vec_set::{vec_set, VecSet};
use super::{AbilitySelection, ControllerID, ControllerParametersMap, PlayerListSelection, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer;

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Framer {
    type ClientRoleState = Framer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Deception => {
                let framer_visits = actor_ref.untagged_night_visits_cloned(game).clone();

                let Some(first_visit) = framer_visits.first() else {return};

                Tags::add_tag(game, TagSetID::Framer(actor_ref), first_visit.target);

                for framed_target in Tags::tagged(game, TagSetID::Framer(actor_ref)){
                    framed_target.set_night_framed(game, true);
                }

                let Some(second_visit) = framer_visits.get(1) else {return};
            
                first_visit.target.set_night_appeared_visits(game, Some(vec![
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
            Priority::Investigative => {
                Tags::set_tagged(
                    game,
                    TagSetID::Framer(actor_ref),
                    Tags::tagged(game, TagSetID::Framer(actor_ref))
                        .into_iter()
                        .filter(|p|
                            !p.all_appeared_visitors(game).iter().any(|visitor| {
                                RoleSet::TownInvestigative.get_roles().contains(&visitor.role(game))
                            })
                        )
                        .collect()
                );
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        
        let frame_players = PlayerReference::all_players(game)
            .filter(|p| 
                p.alive(game) && 
                *p != actor_ref && 
                !InsiderGroupID::in_same_revealed_group(game, actor_ref, *p)
            )
            .collect::<VecSet<_>>();
        
        let grayed_out = 
            actor_ref.ability_deactivated_from_death(game) || 
            Detained::is_detained(game, actor_ref);
        
        
        let frame_controller = ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Framer, 0),
            super::AvailableAbilitySelection::new_player_list(
                frame_players,
                false,
                Some(1)
            ),
            AbilitySelection::new_player_list(vec![]),
            grayed_out,
            Some(crate::game::phase::PhaseType::Obituary),
            false,
            vec_set!(actor_ref)
        );


        let framed_player_exists = if let Some(PlayerListSelection(target)) = game.saved_controllers.get_controller_current_selection_player_list(
            ControllerID::role(actor_ref, Role::Framer, 0)
        ){
            !target.is_empty()
        }else{
            false
        };


        let grayed_out = 
            actor_ref.ability_deactivated_from_death(game) || 
            Detained::is_detained(game, actor_ref) ||
            !framed_player_exists;
        
        let frame_into_controller = ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Framer, 1),
            super::AvailableAbilitySelection::new_player_list(
                PlayerReference::all_players(game).collect(),
                false,
                Some(1)
            ),
            AbilitySelection::new_player_list(vec![]),
            grayed_out,
            Some(crate::game::phase::PhaseType::Obituary),
            false,
            vec_set!(actor_ref)
        );

        frame_controller.combine_overwrite_owned(frame_into_controller)
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