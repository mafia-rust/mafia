use serde::Serialize;

use crate::game::components::insider_group::InsiderGroupID;
use crate::game::role_list::RoleSet;
use crate::game::tag::Tag;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::{Visit, VisitTag};

use crate::game::Game;
use crate::vec_set::VecSet;
use super::{Priority, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer{
    framed_targets: VecSet<PlayerReference>
}

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Framer {
    type ClientRoleState = Framer;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Deception => {
                let framer_visits = actor_ref.untagged_night_visits_cloned(game).clone();

                let Some(first_visit) = framer_visits.first() else {return};

                self.framed_targets.insert(first_visit.target);

                first_visit.target.set_night_framed(game, true);
                for framed_target in self.framed_targets.iter(){
                    framed_target.set_night_framed(game, true);
                }
                self.update_framer_tags(game, actor_ref);
                actor_ref.set_role_state(game, self);

                let Some(second_visit) = framer_visits.get(1) else {return};
            
                first_visit.target.set_night_appeared_visits(game, Some(vec![
                    Visit::new_none(first_visit.target, second_visit.target, false)
                ]));

                //this code erases only the second framer visit
                let mut new_visits = vec![];
                let mut got_first = false;
                for visit in actor_ref.all_night_visits_cloned(game){
                    if visit.tag == VisitTag::None {
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
                self.framed_targets.retain(|p|
                    !p.all_appeared_visitors(game).iter().any(|visitor| {
                        RoleSet::TownInvestigative.get_roles().contains(&visitor.role(game))
                    })
                );

                self.update_framer_tags(game, actor_ref);
                actor_ref.set_role_state(game, self);
            }
            _ => {}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.alive(game) &&
        (
            actor_ref.selection(game).is_empty() &&
            actor_ref != target_ref &&
            target_ref.alive(game) &&
            !InsiderGroupID::in_same_revealed_group(game, actor_ref, target_ref)
        ) || 
        (
            actor_ref.selection(game).len() == 1
        )
    }
    fn convert_selection_to_visits(self, _game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit::new_none(actor_ref, target_refs[0], false), 
                Visit::new_none(actor_ref, target_refs[1], false)
            ]
        } else if target_refs.len() == 1 {
            vec![
                Visit::new_none(actor_ref, target_refs[0], false)
            ]
        } else {
            Vec::new()
        }
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