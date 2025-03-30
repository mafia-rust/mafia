use rand::seq::IteratorRandom;
use serde::Serialize;
use vec1::vec1;

use crate::game::ability_input::{AvailablePlayerListSelection, AvailableRoleOptionSelection, ControllerID};
use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::game_conclusion::GameConclusion;
use crate::game::role_list::{RoleOutline, RoleOutlineOption, RoleOutlineOptionRoles, RoleSet};
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, player::PlayerReference};

use crate::game::visit::{Visit, VisitTag};

use crate::game::Game;
use super::{
    common_role, ControllerParametersMap, Priority, Role, RoleOptionSelection, 
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
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Deception => {
                if !self.convert_charges_remaining {return}

                actor_ref.set_night_visits(game, actor_ref
                    .all_night_visits_cloned(game)
                    .into_iter()
                    .map(|mut v|{
                        if 
                            !InsiderGroupID::in_same_revealed_group(game, actor_ref, v.target) &&
                            v.tag == VisitTag::Role
                        {
                            v.attack = true;
                        }
                        v
                    }
                ).collect());
            },
            Priority::Convert => {
                let visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(target_ref) = visits.first().map(|v| v.target) else {return};

                let role = 
                if let Some(RoleOptionSelection(Some(role))) = game.saved_controllers.get_controller_current_selection_role_option(
                    ControllerID::role(actor_ref, Role::Reeducator, 1)
                ){
                    role
                }else if let Some(role) = Reeducator::default_role(game){
                    role
                }else{
                    return
                };

                let new_state = role.new_state(game);

                if InsiderGroupID::in_same_revealed_group(game, actor_ref, target_ref) {

                    target_ref.set_night_convert_role_to(game, Some(new_state));

                }else if self.convert_charges_remaining && game.day_number() > 1{

                    if target_ref.night_defense(game).can_block(AttackPower::Basic) {
                        actor_ref.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                        return
                    }

                    InsiderGroupID::Mafia.add_player_to_revealed_group(game, target_ref);
                    target_ref.set_win_condition(
                        game,
                        WinCondition::new_loyalist(crate::game::game_conclusion::GameConclusion::Mafia)
                    );
                    target_ref.set_night_convert_role_to(game, Some(new_state));

                    self.convert_charges_remaining = false;
                    actor_ref.set_role_state(game, self);
                }
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
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        common_role::convert_controller_selection_to_visits(game, actor_ref, ControllerID::role(actor_ref, Role::Reeducator, 0), false)
    }
    fn before_initial_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {

        if game.settings.role_list.0.contains(&RoleOutline::new_exact(Role::Reeducator)) {
            return;
        }

        //get random mafia player and turn them info a random town role

        let random_mafia_player = PlayerReference::all_players(game)
            .filter(|p|RoleSet::Mafia.get_roles().contains(&p.role(game)))
            .filter(|p|!RoleSet::MafiaKilling.get_roles().contains(&p.role(game)))
            .filter(|p|*p!=actor_ref)
            .filter(|p|p.role(game)!=Role::Reeducator)
            .choose(&mut rand::rng());

        if let Some(random_mafia_player) = random_mafia_player {

            let random_town_role = RoleOutline { 
                options: vec1![RoleOutlineOption {
                    win_condition: Default::default(),
                    insider_groups: Default::default(),
                    roles: RoleOutlineOptionRoles::RoleSet { role_set: RoleSet::TownCommon }
                }]
            }
                .get_random_role_assignments(
                    &game.settings.enabled_roles,
                    PlayerReference::all_players(game).map(|p|p.role(game)).collect::<Vec<_>>().as_slice()
                ).map(|assignment| assignment.role);

            if let Some(random_town_role) = random_town_role {
                //special case here. I don't want to use set_role because it alerts the player their role changed
                //NOTE: It will still send a packet to the player that their role state updated,
                //so it might be deducable that there is a recruiter
                InsiderGroupID::Mafia.remove_player_from_revealed_group(game, random_mafia_player);
                random_mafia_player.set_win_condition(game, crate::game::win_condition::WinCondition::GameConclusionReached{
                    win_if_any: vec![GameConclusion::Town].into_iter().collect()
                });
                random_mafia_player.set_role_state(game, random_town_role.new_state(game));
                
            }
        }
        
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}

impl Reeducator {
    pub fn default_role(game: &Game) -> Option<Role> {
        RoleSet::MafiaSupport.get_roles().into_iter()
            .filter(|p|game.settings.enabled_roles.contains(p))
            .filter(|p|*p!=Role::Reeducator)
            .choose(&mut rand::rng())
    }
}