use std::time::Duration;

use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::grave::Grave;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::tag::Tag;
use crate::game::win_condition::WinCondition;
use crate::game::Game;

use super::{GetClientRoleState, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Default)]
pub struct Politician{
    pub revealed: bool,
    countdown_started: bool,
    countdown_nomination_started: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Politician {
    type ClientRoleState = ClientRoleState;
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, _target_ref: PlayerReference) {

        if !actor_ref.alive(game) || !game.current_phase().is_day() {
            return;
        }

        game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MayorRevealed { player_index: actor_ref.index() });

        actor_ref.set_role_state(game, Politician{
            revealed: true,
            ..self
        });
        for player in PlayerReference::all_players(game){
            player.push_player_tag(game, actor_ref, Tag::Enfranchised);
        }
        game.count_votes_and_start_trial();
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool{
        game.current_phase().is_day() &&
        !self.revealed &&
        actor_ref == target_ref &&
        actor_ref.alive(game) &&
        PhaseType::Night != game.current_phase().phase()
    }

    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if
            actor_ref.alive(game) &&
            PlayerReference::all_players(game)
                .filter(|p|p.alive(game))
                .filter(|p|p.keeps_game_running(game))
                .all(|p|
                    !p.win_condition(game).is_loyalist_for(GameConclusion::Town)
                )

        {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }

        if self.countdown_started {
            match phase {
                PhaseType::Nomination => {
                    self.countdown_nomination_started = true;
                    actor_ref.set_role_state(game, self);
                },
                PhaseType::Dusk => {
                    if self.countdown_nomination_started {
                        Politician::kill_all(game);
                    }
                },
                _ => {
                    game.phase_machine.time_remaining = Duration::from_secs(0);
                }
            }
        }
    }
    
    fn on_any_death(mut self, game: &mut Game, actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
        if _dead_player_ref == actor_ref || self.countdown_started {
            return; 
        }
        self.countdown_started = actor_ref.alive(game) &&
            PlayerReference::all_players(game)
            .filter(|p|*p != actor_ref)
            .filter(|p|p.keeps_game_running(game))
            .filter(|p|p.alive(game))
            .any(|player| {
                player.win_condition(game).is_loyalist_for(GameConclusion::Town)
            });
        
        if self.countdown_started {
            Politician::start_countdown(&mut self, game);
        }

        actor_ref.set_role_state(game, self);
    }

    fn default_win_condition(self) -> crate::game::win_condition::WinCondition where RoleState: From<Self> {
        WinCondition::GameConclusionReached{win_if_any: vec![GameConclusion::Politician].into_iter().collect()}
    }
}

impl GetClientRoleState<ClientRoleState> for Politician {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}

impl Politician {
    fn start_countdown(&mut self, game: &mut Game){
        game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::PoliticianCountdownStarted);
        
        if game.current_phase().phase() != PhaseType::Nomination {
            game.phase_machine.time_remaining = Duration::from_secs(0);
        }
        self.countdown_started = true;
    }

    fn kill_all(game: &mut Game){
        for player in PlayerReference::all_players(game){
            if player.alive(game) && !player.win_condition(game).is_loyalist_for(GameConclusion::Politician) {
                player.die(game, Grave::from_player_leave_town(game, player));
            }
        }
    }
}