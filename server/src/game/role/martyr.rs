use std::vec;

use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::{Grave, GraveDeathCause, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl, Role, RoleState};

#[derive(PartialEq, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Martyr {
    pub state: MartyrState
}

#[derive(PartialEq, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum MartyrState {
    Won,
    StillPlaying {
        bullets: u8
    },
    LeftTown
}

impl Default for Martyr {
    fn default() -> Self {
        Self{
            state: MartyrState::StillPlaying { bullets: 5 }
        }
    }
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

impl RoleStateImpl for Martyr {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}

    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}
        let MartyrState::StillPlaying { bullets } = self.state else {return};
        if bullets == 0 {return}

        if let Some(visit) = actor_ref.night_visits(game).first() {
            let target_ref = visit.target;
            if target_ref.night_jailed(game){
                actor_ref.push_night_message(game, ChatMessage::TargetJailed);
                return;
            }

            self.state = MartyrState::StillPlaying { bullets: bullets.saturating_sub(1) };

            if target_ref == actor_ref {
                if target_ref.try_night_kill(actor_ref, game, GraveKiller::Suicide, 1, true) {
                    self.state = MartyrState::Won;
                }
            } else {
                target_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Martyr), 1, true);
            }
        };

        actor_ref.set_role_state(game, RoleState::Martyr(self));
    }

    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref == target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref.alive(game) && 
        match self.state {
            MartyrState::StillPlaying { bullets } => bullets != 0,
            _ => false
        }
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, _game: &Game, _actor_ref: PlayerReference) -> bool {
        self.state == MartyrState::Won
    }
    fn on_phase_start(self,  game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase == PhaseType::Morning && matches!(self.state, MartyrState::StillPlaying {..}) {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessage::MartyrFailed);
        }

        if phase == PhaseType::Morning && actor_ref.alive(game) && matches!(self.state, MartyrState::StillPlaying { bullets: 0 }) {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref), true);
        }
    }
    fn on_role_creation(self,  game: &mut Game, actor_ref: PlayerReference) {
        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::MartyrRevealed { martyr: actor_ref.index() });
        for player in PlayerReference::all_players(game){
            player.insert_role_label(game, actor_ref, Role::Martyr);
        }
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference) {
        let left_town = game.graves.iter().any(|grave| grave.player == dead_player_ref.index() && grave.death_cause == GraveDeathCause::LeftTown);
        if dead_player_ref == actor_ref && !left_town {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessage::MartyrWon);
            
            for player in PlayerReference::all_players(game) {
                if player == actor_ref {continue}
                if !player.alive(game) {continue}
                if player.defense(game) >= 3 {continue}
                player.die(game, Grave::from_player_suicide(game, player), true);
            }
    
            actor_ref.set_role_state(game, RoleState::Martyr(Martyr {
                state: MartyrState::Won
            }));
        }
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
