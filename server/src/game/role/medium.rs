use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::{PhaseType, PhaseState};
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl, RoleState};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Medium{
    pub seances_remaining: u8,
    pub seanced_target: Option<PlayerReference>
}
impl Default for Medium{
    fn default() -> Self {
        Self { seances_remaining: 2, seanced_target: None}
    }
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownSupport;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Medium {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::Town}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {
    }
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.seanced_target {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, RoleState::Medium(Medium { seanced_target: None, seances_remaining: self.seances_remaining}));
            } else {
                actor_ref.set_role_state(game, RoleState::Medium(Medium { seanced_target: Some(target_ref), seances_remaining: self.seances_remaining }))
            }
        } else {
            actor_ref.set_role_state(game, RoleState::Medium(Medium { seanced_target: Some(target_ref), seances_remaining: self.seances_remaining }))
        }
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        game.current_phase().is_day() &&
        self.seances_remaining > 0 && 
        actor_ref != target_ref &&
        !actor_ref.alive(game) && target_ref.alive(game) && 
        game.current_phase().phase() != PhaseType::Night
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        if !actor_ref.alive(game){
            let mut out = vec![ChatGroup::Dead];
            if game.current_phase().phase() == PhaseType::Night{
                out.push(ChatGroup::Seance);
            }
            return out;
        }
        if actor_ref.night_silenced(game){
            return vec![];
        }
    
        match game.current_phase() {
            PhaseState::Morning => vec![],
            PhaseState::Discussion 
            | PhaseState::Voting {..}
            | PhaseState::Judgement {..} 
            | PhaseState::Evening {..} => vec![ChatGroup::All],
            &PhaseState::Testimony { player_on_trial, .. } => {
                if player_on_trial == actor_ref {
                    vec![ChatGroup::All]
                } else {
                    vec![]
                }
            },
            PhaseState::Night => {
                let mut out = vec![];
                if PlayerReference::all_players(game).into_iter()
                    .any(|med|{
                        if let RoleState::Medium(medium_state) = med.role_state(game){
                            if Some(actor_ref) == medium_state.seanced_target{
                                return true;
                            }
                        }
                        false
                    })
                {
                    out.push(ChatGroup::Seance);
                }
    
    
                let mut jail_or_night_chats = if actor_ref.night_jailed(game){
                    vec![ChatGroup::Jail]
                } else {
                    vec![ChatGroup::Dead]
                };
                out.append(&mut jail_or_night_chats);
                out
            },
        }
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);

        if game.current_phase().is_night() && actor_ref.alive(game) {
            out.push(ChatGroup::Dead);
        }
        if game.current_phase().is_night() && !actor_ref.alive(game) {
            out.push(ChatGroup::Seance);
        }
        out
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Morning => {
                self.seanced_target = None;
                actor_ref.set_role_state(game, RoleState::Medium(self));
            },
            PhaseType::Night => {
                if let Some(seanced) = self.seanced_target {
                    if seanced.alive(game) && !actor_ref.alive(game){
                
                        actor_ref.add_chat_message(game, 
                            ChatMessage::MediumSeance{ player: seanced.index() }
                        );
                        seanced.add_chat_message(game, 
                            ChatMessage::MediumSeance{ player: seanced.index() }
                        );
                        self.seances_remaining -= 1;
                    }
                }
                actor_ref.set_role_state(game, RoleState::Medium(self));
            },
            _=>{}
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
