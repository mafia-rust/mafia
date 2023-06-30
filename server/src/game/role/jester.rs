
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::{PhaseType, PhaseState};
use crate::game::player::PlayerReference;
use crate::game::role::RoleState;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::verdict::Verdict;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCK_IMMUNE: bool = false;
pub(super) const CONTROL_IMMUNE: bool = false;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::NeutralEvil;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::None;
pub(super) const TEAM: Option<Team> = None;

#[derive(Clone, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Jester {
    lynched_yesterday: bool
}

impl RoleStateImpl for Jester {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::TopPriority {return;}
        if actor_ref.night_alive_tonight(game) {return;}
    
        if !self.lynched_yesterday {return}
        
        let all_killable_players: Vec<PlayerReference> = PlayerReference::all_players(game)
            .into_iter()
            .filter(|player_ref|{
                player_ref.night_alive_tonight(game) &&
                *player_ref != actor_ref &&
                player_ref.verdict(game) == Verdict::Guilty
            }).collect();
    
        let visit: Visit = match actor_ref.night_visits(game).first() {
            Some(v) => v.clone(),
            None => {
                //get random player from list
                let target_ref = all_killable_players.choose(&mut rand::thread_rng());
    
                let Some(target_ref) = target_ref else {return};
                Visit{
                    target: *target_ref,
                    astral: true,
                    attack: true,
                }
            },
        };
    
        if visit.target.try_night_kill(actor_ref, game, crate::game::grave::GraveKiller::Role(super::Role::Jester), 3) {
            if let Some(note) = actor_ref.death_note(game) {
                visit.target.push_night_grave_death_notes(game, note.clone());
            }
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        actor_ref.chosen_targets(game).is_empty() &&
        !actor_ref.alive(game) &&
        target_ref.alive(game) &&
        target_ref.verdict(game) != Verdict::Innocent &&
        self.lynched_yesterday
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
        
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, true, true)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        match game.current_phase() {
            &PhaseState::Evening { player_on_trial: Some(player_on_trial) } => {
                if player_on_trial == actor_ref {
                    actor_ref.set_role_state(game, RoleState::Jester(Jester { lynched_yesterday: true }))
                }
            }
            PhaseState::Morning => {
                actor_ref.set_role_state(game, RoleState::Jester(Jester { lynched_yesterday: false }))
            }
            _ => {}
        }
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        if 
            actor_ref == dead_player_ref && 
            game.current_phase().phase() == PhaseType::Evening
        {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessage::JesterWon);
        }
    }
}
