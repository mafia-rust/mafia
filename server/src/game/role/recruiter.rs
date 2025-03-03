
use rand::seq::IteratorRandom;
use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::components::mafia_recruits::MafiaRecruits;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;
use crate::game::game_conclusion::GameConclusion;
use crate::game::role_list::{RoleOutline, RoleOutlineOption, RoleOutlineOptionRoles, RoleSet};
use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set;
use super::godfather::Godfather;
use super::{
    common_role, AbilitySelection, AvailableAbilitySelection, ControllerID,
    ControllerParametersMap, IntegerSelection, Priority, Role, RoleStateImpl
};

use vec1::vec1;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recruiter{
    pub recruits_remaining: u8,
}

impl Default for Recruiter {
    fn default() -> Self {
        Self {
            recruits_remaining: 3,
        }
    }
}



pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Recruiter {
    type ClientRoleState = Recruiter;
    fn new_state(game: &Game) -> Self {
        Self{
            recruits_remaining: game.num_players().div_ceil(5),
            ..Self::default()
        }
    }
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        let choose_attack = if let Some(IntegerSelection(x)) = game.saved_controllers.get_controller_current_selection_integer(
            ControllerID::role(actor_ref, Role::Recruiter, 1)
        ){x==0}else{true};

        if choose_attack{
            if !game.attack_convert_abilities_enabled() {return}
        }else{
            if self.recruits_remaining == 0 {return}
        }

        match priority {
            Priority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    if Recruiter::night_ability(self.clone(), game, actor_ref, visit.target) {
                        if choose_attack {
                            actor_ref.set_role_state(game, Recruiter{recruits_remaining: self.recruits_remaining.saturating_add(1), ..self})
                        }else{
                            actor_ref.set_role_state(game, Recruiter{recruits_remaining: self.recruits_remaining.saturating_sub(1), ..self});
                        }
                    }
                }
            },
            _ => {return}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {

        let choose_attack = if let Some(IntegerSelection(x)) = game.saved_controllers.get_controller_current_selection_integer(
            ControllerID::role(actor_ref, Role::Recruiter, 1)
        ){x==0}else{true};

        common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            (!choose_attack && self.recruits_remaining <= 0) || (choose_attack && !game.attack_convert_abilities_enabled()),
            ControllerID::role(actor_ref, Role::Recruiter, 0)
        ).combine_overwrite_owned(
            ControllerParametersMap::new_controller_fast(
                game,
                ControllerID::role(actor_ref, Role::Recruiter, 1),
                AvailableAbilitySelection::new_integer(0, if self.recruits_remaining > 0 {1} else {0}),
                AbilitySelection::new_integer(0),
                actor_ref.ability_deactivated_from_death(game),
                None,
                false,
                vec_set![actor_ref],
            )
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Recruiter, 0),
            false
        )
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        Godfather::pass_role_state_down(game, actor_ref, dead_player_ref, self);
    }
    fn before_initial_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {

        if game.settings.role_list.0.contains(&RoleOutline::new_exact(Role::Recruiter)) {
            return;
        }

        //get random mafia player and turn them info a random town role

        let random_mafia_player = PlayerReference::all_players(game)
            .filter(|p|RoleSet::Mafia.get_roles().contains(&p.role(game)))
            .filter(|p|*p!=actor_ref)
            .choose(&mut rand::rng());

        if let Some(random_mafia_player) = random_mafia_player {

            let random_town_role = RoleOutline {options: vec1![RoleOutlineOption {
                win_condition: Default::default(), 
                insider_groups: Default::default(), 
                roles: RoleOutlineOptionRoles::RoleSet{ role_set: RoleSet::TownCommon } 
            }]}.get_random_role_assignments(
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

impl Recruiter {
    /// returns true if target_ref is killed when trying to kill
    /// returns true if target_ref is recruited when trying to recruit
    pub fn night_ability(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let choose_attack = if let Some(IntegerSelection(x)) = game.saved_controllers.get_controller_current_selection_integer(
            ControllerID::role(actor_ref, Role::Recruiter, 1)
        ){x==0}else{true};

        if choose_attack {
            target_ref.try_night_kill_single_attacker(
                actor_ref,
                game,
                GraveKiller::RoleSet(RoleSet::Mafia),
                AttackPower::Basic,
                false
            )
        }else{
            if AttackPower::Basic.can_pierce(target_ref.defense(game)) {
                MafiaRecruits::recruit(game, target_ref)
            }else{
                false
            }
        }
    }
}
