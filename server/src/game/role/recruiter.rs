use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::mafia_recruits::MafiaRecruits;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{Faction, RoleOutline, RoleOutlineOption};
use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, Role, RoleState, RoleStateImpl};

use vec1::vec1;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recruiter{
    pub backup: Option<PlayerReference>,
    pub recruits_remaining: u8,
    pub action: RecruiterAction,
}

impl Default for Recruiter {
    fn default() -> Self {
        Self {
            backup: None,
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

        let mut ability_successful = false;
        
        if priority != Priority::Kill {return}
        match self.action {
            RecruiterAction::Recruit => {
                if self.recruits_remaining == 0 {return}
            },
            RecruiterAction::Kill => {
                if game.day_number() == 1 {return}
            },
        }
        
        if actor_ref.night_blocked(game) {
            if let Some(backup) = self.backup {

                let mut visits = backup.night_visits(game).clone();
                if let Some(visit) = visits.first_mut(){
                    visit.attack = self.action == RecruiterAction::Kill;
                    game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessageVariant::GodfatherBackupKilled { backup: backup.index() });
                    ability_successful = Recruiter::night_ability(self.clone(), game, backup, visit.target);
                    
                }
                backup.set_night_visits(game, visits);
            }
            
        } else if let Some(visit) = actor_ref.night_visits(game).first(){
            ability_successful = Recruiter::night_ability(self.clone(), game, actor_ref, visit.target);
        }

        if ability_successful {
            match self.action {
                RecruiterAction::Recruit => actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{recruits_remaining: self.recruits_remaining-1, ..self})),
                RecruiterAction::Kill => actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{recruits_remaining: self.recruits_remaining+1, ..self})),
            }
        }
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.backup {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{backup: None, ..self}));
            } else {
                actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{backup: Some(target_ref), ..self}));
            }
        } else {
            actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{backup: Some(target_ref), ..self}));
        }

        let RoleState::Recruiter(Recruiter { backup, .. }) = *actor_ref.role_state(game) else {
            unreachable!("Role was just set to Recruiter");
        };

        game.add_message_to_chat_group(ChatGroup::Mafia, ChatMessageVariant::GodfatherBackup { backup: backup.map(|p|p.index()) });

        for player_ref in PlayerReference::all_players(game){
            if player_ref.role(game).faction() != Faction::Mafia{
                continue;
            }
            player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
        }

        if let Some(backup) = backup {
            for player_ref in PlayerReference::all_players(game){
                if player_ref.role(game).faction() != Faction::Mafia {
                    continue;
                }
                player_ref.push_player_tag(game, backup, Tag::GodfatherBackup);
            }
        }
        
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        actor_ref.alive(game) && target_ref.alive(game) &&
        target_ref.role(game).faction() == Faction::Mafia
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

        if actor_ref == dead_player_ref {
            let Some(backup) = self.backup else {return};

            actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{backup: None, ..self.clone()}));
            for player_ref in PlayerReference::all_players(game){
                if player_ref.role(game).faction() != Faction::Mafia{
                    continue;
                }
                player_ref.remove_player_tag_on_all(game, Tag::GodfatherBackup);
            }
            
            if !backup.alive(game){return}

            //convert backup to godfather
            backup.set_role_and_win_condition_and_revealed_group(game, RoleState::Recruiter(Recruiter{backup: None, ..self}));
        }
        else if self.backup.is_some_and(|p|p == dead_player_ref) {
            actor_ref.set_role_state(game, RoleState::Recruiter(Recruiter{backup: None, ..self}));
        }
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
            .filter(|p|p.role(game).faction() == Faction::Mafia)
            .filter(|p|*p!=actor_ref)
            .choose(&mut rand::thread_rng());

        if let Some(random_mafia_player) = random_mafia_player {

            let random_town_role = RoleOutline::RoleOutlineOptions { options: vec1![RoleOutlineOption::Faction{ faction: Faction::Town }] }
                .get_random_role(
                    &game.settings.enabled_roles,
                    PlayerReference::all_players(game).map(|p|p.role(game)).collect::<Vec<_>>().as_slice()
                );

            if let Some(random_town_role) = random_town_role {
                //special case here. I don't want to use set_role because it alerts the player their role changed
                //NOTE: It will still send a packet to the player that their role state updated,
                //so it might be deducable that there is a recruiter
                random_mafia_player.set_role_state(game, random_town_role.default_state());
            }
        }
        
    }
    fn default_revealed_groups(self) -> std::collections::HashSet<crate::game::components::revealed_group::RevealedGroupID> {
        vec![
            crate::game::components::revealed_group::RevealedGroupID::Mafia
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