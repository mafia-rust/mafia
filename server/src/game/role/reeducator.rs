use rand::seq::IteratorRandom;
use serde::Serialize;

use crate::game::ability_input::{AvailablePlayerListSelection, AvailableRoleOptionSelection, ControllerID};
use crate::game::attack_power::AttackPower;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::event::on_midnight::{MidnightVariables, OnMidnightPriority};
use crate::game::role_list::RoleSet;
use crate::game::grave::GraveKiller;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::{Visit, VisitTag};

use crate::game::Game;
use super::{
    common_role, ControllerParametersMap, Role, RoleOptionSelection, 
    RoleStateImpl
};


#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reeducator{
    convert_charges_remaining: bool,
}
impl Default for Reeducator{
    fn default() -> Self {
        Self {
            convert_charges_remaining: true,
        }
    }
}

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Reeducator {
    type ClientRoleState = Reeducator;
    fn on_midnight(mut self, game: &mut Game, midnight_variables: &mut MidnightVariables, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        match priority {
            OnMidnightPriority::Roleblock => {
                if !self.convert_charges_remaining || game.day_number() <= 1 {return}

                let mut converting = false;

                actor_ref.set_night_visits(
                    game, 
                    actor_ref
                        .all_night_visits_cloned(game)
                        .into_iter()
                        .map(|mut v|{
                            if 
                                !InsiderGroupID::in_same_revealed_group(game, actor_ref, v.target) &&
                                v.tag == VisitTag::Role
                            {
                                v.attack = true;
                                converting = true;
                            }
                            v
                        })
                        .collect()
                );

                if converting {
                    for fellow_insider in InsiderGroupID::Mafia.players(game).clone().iter(){
                        fellow_insider.roleblock(game, midnight_variables, true);
                    }
                }
            },
            OnMidnightPriority::Convert => {
                let visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = visits.first() else {return};

                let role = 
                if let Some(RoleOptionSelection(Some(role))) = ControllerID::role(actor_ref, Role::Reeducator, 1)
                    .get_role_option_selection(game)
                {
                    *role
                }else if let Some(role) = Reeducator::default_role(game){
                    role
                }else{
                    return
                };

                let new_state = role.new_state(game);

                if visit.attack {
                    if self.convert_charges_remaining && game.day_number() > 1 {
                        actor_ref.try_night_kill_single_attacker(
                            actor_ref,
                            game,
                            midnight_variables,
                            GraveKiller::RoleSet(RoleSet::Mafia),
                            AttackPower::ProtectionPiercing,
                            false,
                        );
    
                        InsiderGroupID::Mafia.add_player_to_revealed_group(game, visit.target);
                        visit.target.set_win_condition(
                            game,
                            WinCondition::new_loyalist(crate::game::game_conclusion::GameConclusion::Mafia)
                        );
                        visit.target.set_night_convert_role_to(midnight_variables, Some(new_state));
    
                        self.convert_charges_remaining = false;
                        actor_ref.set_role_state(game, self);
                    }
                }else{
                    visit.target.set_night_convert_role_to(midnight_variables, Some(new_state));
                };
            },
            _ => {}
        }                
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::combine([
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Reeducator, 0))
                .available_selection(AvailablePlayerListSelection {
                    available_players: PlayerReference::all_players(game)
                        .filter(|player| 
                            player.alive(game) &&
                            (
                                InsiderGroupID::in_same_revealed_group(game, actor_ref, *player) || 
                                (game.day_number() > 1 && self.convert_charges_remaining)
                            )
                        )
                        .collect(),
                    can_choose_duplicates: false,
                    max_players: Some(1)
                })
                .night_typical(actor_ref)
                .build_map(),
            ControllerParametersMap::builder(game)
                .id(ControllerID::role(actor_ref, Role::Reeducator, 1))
                .available_selection(AvailableRoleOptionSelection(
                    RoleSet::MafiaSupport.get_roles().into_iter()
                        .filter(|p|game.settings.enabled_roles.contains(p))
                        .filter(|p|*p!=Role::Reeducator)
                        .map(Some)
                        .collect()
                ))
                .default_selection(RoleOptionSelection(Reeducator::default_role(game)))
                .allow_players([actor_ref])
                .build_map()
        ])
    }
    // Unlike other conversion roles, its visit isn't tagged as an attack.
    // This is because if the target is syndicate then it is converted without an attack
    // After transportation, it becomes an attack
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(
            game, 
            actor_ref, 
            ControllerID::role(actor_ref, Role::Reeducator, 0),
            false
        )
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
    fn on_player_roleblocked(self, _game: &mut Game, _actor_ref: PlayerReference, _player: PlayerReference, _invisible: bool) {}
}

impl Reeducator {
    pub fn default_role(game: &Game) -> Option<Role> {
        RoleSet::MafiaSupport.get_roles().into_iter()
            .filter(|p|game.settings.enabled_roles.contains(p))
            .filter(|p|*p!=Role::Reeducator)
            .choose(&mut rand::rng())
    }
}