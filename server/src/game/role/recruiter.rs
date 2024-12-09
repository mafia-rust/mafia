use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::components::mafia_recruits::MafiaRecruits;
use crate::game::components::insider_group::InsiderGroupID;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::game_conclusion::GameConclusion;
use crate::game::role_list::{RoleOutline, RoleOutlineOption, RoleSet};
use crate::game::visit::Visit;

use crate::game::Game;
use super::godfather::Godfather;
use super::{Priority, Role, RoleState, RoleStateImpl};

use vec1::vec1;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recruiter{
    pub recruits_remaining: u8,
    pub action: RecruiterAction,
}

impl Default for Recruiter {
    fn default() -> Self {
        Self {
            recruits_remaining: 3,
            action: RecruiterAction::Recruit
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum RecruiterAction{
    Recruit,
    Kill
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Recruiter {
    type ClientRoleState = Recruiter;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {

        match self.action {
            RecruiterAction::Recruit => {
                if self.recruits_remaining == 0 {return}
            },
            RecruiterAction::Kill => {
                if game.day_number() == 1 {return}
            },
        }

        match priority {
            Priority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    if Recruiter::night_ability(self.clone(), game, actor_ref, visit.target) {
                        match self.action {
                            RecruiterAction::Recruit => actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{recruits_remaining: self.recruits_remaining-1, ..self})),
                            RecruiterAction::Kill => actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{recruits_remaining: self.recruits_remaining+1, ..self})),
                        }
                    }
                }
            },
            _ => {return}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref) &&
        !MafiaRecruits::is_recruited(game, target_ref) &&
        match self.action {
            RecruiterAction::Recruit => {
                self.recruits_remaining > 0
            },
            RecruiterAction::Kill => {
                game.day_number() > 1
            },
        }
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, match self.action {
            RecruiterAction::Recruit => false,
            RecruiterAction::Kill => true,
        })
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        Godfather::pass_role_state_down(game, actor_ref, dead_player_ref, self);
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        if phase == PhaseType::Night && self.recruits_remaining == 0{
            self.action = RecruiterAction::Kill;
            actor_ref.set_role_state(game, RoleState::Recruiter(self))
        }
    }
    fn before_initial_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {

        if game.settings.role_list.0.contains(&RoleOutline::new_exact(Role::Recruiter)) {
            return;
        }

        //get random mafia player and turn them info a random town role

        let random_mafia_player = PlayerReference::all_players(game)
            .filter(|p|RoleSet::Mafia.get_roles().contains(&p.role(game)))
            .filter(|p|*p!=actor_ref)
            .choose(&mut rand::thread_rng());

        if let Some(random_mafia_player) = random_mafia_player {

            let random_town_role = RoleOutline::RoleOutlineOptions { options: vec1![RoleOutlineOption::RoleSet { role_set: RoleSet::TownCommon }] }
                .get_random_role(
                    &game.settings.enabled_roles,
                    PlayerReference::all_players(game).map(|p|p.role(game)).collect::<Vec<_>>().as_slice()
                );

            if let Some(random_town_role) = random_town_role {
                //special case here. I don't want to use set_role because it alerts the player their role changed
                //NOTE: It will still send a packet to the player that their role state updated,
                //so it might be deducable that there is a recruiter
                InsiderGroupID::Mafia.remove_player_from_revealed_group(game, random_mafia_player);
                random_mafia_player.set_win_condition(game, crate::game::win_condition::WinCondition::GameConclusionReached{
                    win_if_any: vec![GameConclusion::Town].into_iter().collect()
                });
                random_mafia_player.set_role_state(game, random_town_role.default_state());
                
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
        match self.action {
            RecruiterAction::Recruit => {
                if AttackPower::Basic.can_pierce(target_ref.defense(game)) {
                    MafiaRecruits::recruit(game, target_ref)
                }else{
                    false
                }
            },
            RecruiterAction::Kill => {
                target_ref.try_night_kill_single_attacker(
                    actor_ref,
                    game,
                    GraveKiller::RoleSet(RoleSet::Mafia),
                    AttackPower::Basic,
                    false
                )
            },
        }
    }
}