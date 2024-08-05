
use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::player_group::PlayerGroup;
use crate::game::grave::GraveReference;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleState, RoleStateImpl};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Journalist {
    pub public: bool,
    pub journal: String,
    pub interviewed_target: Option<PlayerReference>, 
}
impl Default for Journalist {
    fn default() -> Self {
        Journalist {
            public: true,
            journal: "".to_string(),
            interviewed_target: None,
        }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Journalist {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if 
            priority == Priority::Investigative &&
            self.public && 
            actor_ref.alive(game) &&
            !actor_ref.night_blocked(game) &&
            !actor_ref.night_silenced(game)
        {
            game.add_message(PlayerGroup::All, ChatMessageVariant::JournalistJournal { journal: self.journal.clone()});    
        }
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
        if let Some(old_target_ref) = self.interviewed_target {
            if old_target_ref == target_ref {
                actor_ref.set_role_state(game, RoleState::Journalist(Journalist { interviewed_target: None, ..self}));
            } else {
                actor_ref.set_role_state(game, RoleState::Journalist(Journalist { interviewed_target: Some(target_ref), ..self }));
            }
        } else {
            actor_ref.set_role_state(game, RoleState::Journalist(Journalist { interviewed_target: Some(target_ref), ..self }));
        }
    }
    fn can_select(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        game.current_phase().is_day() &&
        actor_ref != target_ref &&
        actor_ref.alive(game) && target_ref.alive(game)
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, 
            if 
                game.current_phase().is_night() &&
                actor_ref.alive(game) &&
                self.interviewed_target.map_or_else(||false, |p|p.alive(game))
            {
                vec![PlayerGroup::Interview]
            }else{
                vec![]
            }
        )
    }
    fn get_current_receive_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        let mut out = crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref);
        if 
            game.current_phase().is_night() &&
            actor_ref.alive(game) &&
            self.interviewed_target.map_or_else(||false, |p|p.alive(game))
        {
            out.push(PlayerGroup::Interview);
        }
        out
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        match phase {
            PhaseType::Night => {
                if let Some(interviewed_target_ref) = self.interviewed_target {
                    if interviewed_target_ref.alive(game) && actor_ref.alive(game){
                        actor_ref.add_private_chat_message(game, 
                            ChatMessageVariant::YouAreInterviewingPlayer { player_index: interviewed_target_ref.index() }
                        );

                        let mut message_sent = false;
                        for chat_group in interviewed_target_ref.get_current_send_chat_groups(game){
                            match chat_group {
                                PlayerGroup::All | PlayerGroup::Jail | PlayerGroup::Interview | PlayerGroup::Dead => {},
                                PlayerGroup::Mafia | PlayerGroup::Cult  => {
                                    game.add_message(
                                        chat_group,
                                        ChatMessageVariant::PlayerIsBeingInterviewed { player_index: interviewed_target_ref.index() }
                                    );
                                    message_sent = true;
                                },
                            }
                        }
                        if !message_sent {
                            interviewed_target_ref.add_private_chat_message(game, 
                                ChatMessageVariant::PlayerIsBeingInterviewed { player_index: interviewed_target_ref.index() }
                            );
                        }

                    }else{
                        self.interviewed_target = None;
                        actor_ref.set_role_state(game, RoleState::Journalist(Journalist{interviewed_target: None, ..self}));
                    }
                }
            },
            PhaseType::Obituary => {
                self.interviewed_target = None;
                actor_ref.set_role_state(game, RoleState::Journalist(Journalist{interviewed_target: None, ..self}));
            },
            _ => {}
        }
        
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference) {
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}