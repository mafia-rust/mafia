use rand::seq::IteratorRandom;
use serde::Serialize;

use crate::game::ability_input::{AvailablePlayerListSelection, AvailableRoleOptionSelection, ControllerID};
use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::event::on_midnight::OnMidnightPriority;
use crate::game::grave::GraveKiller;
use crate::game::role_list::RoleSet;
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
    fn on_midnight(mut self, game: &mut Game, actor_ref: PlayerReference, priority: OnMidnightPriority) {
        match priority {
            OnMidnightPriority::Deception => {
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
            OnMidnightPriority::Convert => {
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

                    actor_ref.try_night_kill_single_attacker(
                        actor_ref,
                        game,
                        GraveKiller::RoleSet(RoleSet::Mafia),
                        AttackPower::ProtectionPiercing,
                        false,
                    );

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
}

impl Reeducator {
    pub fn default_role(game: &Game) -> Option<Role> {
        RoleSet::MafiaSupport.get_roles().into_iter()
            .filter(|p|game.settings.enabled_roles.contains(p))
            .filter(|p|*p!=Role::Reeducator)
            .choose(&mut rand::rng())
    }
}