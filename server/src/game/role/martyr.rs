use std::vec;

use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

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
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Martyr {
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}
        let MartyrState::StillPlaying { bullets } = self.state else {return};
        if bullets == 0 {return}

        if let Some(visit) = actor_ref.night_visits(game).first() {
            let target_ref = visit.target;

            self.state = MartyrState::StillPlaying { bullets: bullets.saturating_sub(1) };

            if target_ref == actor_ref {
                if target_ref.try_night_kill(actor_ref, game, GraveKiller::Suicide, 1, true) {
                    self.state = MartyrState::Won;
                }
            } else {
                target_ref.try_night_kill(actor_ref, game, GraveKiller::Role{value: Role::Martyr}, 1, true);
            }
        };

        actor_ref.set_role_state(game, RoleState::Martyr(self));
    }

    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref == target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game) && 
        match self.state {
            MartyrState::StillPlaying { bullets } => bullets != 0,
            _ => false
        }
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
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
        if phase == PhaseType::Obituary && matches!(self.state, MartyrState::StillPlaying {..}) {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MartyrFailed);
        }

        if phase == PhaseType::Obituary && actor_ref.alive(game) && matches!(self.state, MartyrState::StillPlaying { bullets: 0 }) {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }
    fn on_role_creation(self,  game: &mut Game, actor_ref: PlayerReference) {
        game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MartyrRevealed { martyr: actor_ref.index() });
        for player in PlayerReference::all_players(game){
            player.insert_role_label(game, actor_ref);
        }
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference) {
        let left_town = game.graves.iter().any(|grave| 
            grave.player == dead_player_ref &&
            if let GraveInformation::Normal { death_cause, .. } = &grave.information {
                death_cause == &GraveDeathCause::LeftTown
            } else {false}
        );

        if dead_player_ref == actor_ref && !left_town {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MartyrWon);
            
            for player in PlayerReference::all_players(game) {
                if player == actor_ref {continue}
                if !player.alive(game) {continue}
                if player.defense(game) >= 3 {continue}
                player.die(game, Grave::from_player_suicide(game, player));
            }
    
            actor_ref.set_role_state(game, RoleState::Martyr(Martyr {
                state: MartyrState::Won
            }));
        }
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave: crate::game::grave::GraveReference) {
        
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
