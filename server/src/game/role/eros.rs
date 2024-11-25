use serde::{Deserialize, Serialize};

use crate::game::attack_power::AttackPower;
use crate::game::role_list::RoleSet;
use crate::game::{attack_power::DefensePower, components::love_linked::LoveLinked};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{InsiderGroupID, Priority, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Eros{
    pub action: ErosAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase")]
pub enum ErosAction{
    #[default] LoveLink,
    Kill,
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Eros {
    type ClientRoleState = Eros;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match (priority, self.action) {
            (Priority::Kill, ErosAction::Kill) => {
                if game.day_number() == 1 {return}
                let actor_visits = actor_ref.night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    let target_ref = visit.target;
            
                    target_ref.try_night_kill_single_attacker(
                        actor_ref, game, GraveKiller::RoleSet(RoleSet::Mafia), AttackPower::Basic, false
                    );
                }
            }
            (Priority::Cupid, ErosAction::LoveLink) => {
                let visits = actor_ref.night_visits_cloned(game);

                let Some(first_visit) = visits.get(0) else {return};
                let Some(second_visit) = visits.get(1) else {return};
                
                let player1 = first_visit.target;
                let player2 = second_visit.target;

                LoveLinked::add_love_link(game, player1, player2);
            },
            _ => ()
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let selected = actor_ref.selection(game);

        actor_ref != target_ref &&
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        match self.action {
            ErosAction::LoveLink => {
                selected.len() < 2 &&
                selected.iter().all(|&p| p != target_ref)
            },
            ErosAction::Kill => {
                game.day_number() > 1 &&
                !InsiderGroupID::in_same_revealed_group(game, actor_ref, target_ref) &&
                selected.is_empty()
            },
        }
    }
    fn convert_selection_to_visits(self, _game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        match self.action {
            ErosAction::LoveLink => {
                if target_refs.len() == 2 {
                    vec![
                        Visit::new_none(actor_ref, target_refs[0], false),
                        Visit::new_none(actor_ref, target_refs[1], false),
                    ]
                } else {
                    Vec::new()
                }
            },
            ErosAction::Kill => {
                if !target_refs.is_empty() {
                    vec![
                        Visit::new_none(actor_ref, target_refs[0], true)
                    ]
                } else {
                    Vec::new()
                }
            }
        }
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}