pub mod grave;
pub mod phase;
pub mod player;
pub mod chat;
pub mod role;
pub mod visit;
pub mod verdict;
pub mod role_list;
pub mod settings;
pub mod end_game_condition;
pub mod team;
pub mod available_buttons;
pub mod on_client_message;
pub mod tag;
pub mod recruit;

use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::lobby::LobbyPlayer;
use crate::packet::{ToClientPacket, GameOverReason};
use chat::{ChatMessage, ChatGroup};
use player::PlayerReference;
use role_list::{RoleOutline, create_random_roles};
use player::Player;
use phase::PhaseStateMachine;
use settings::Settings;
use grave::Grave;

use self::phase::PhaseState;
use self::role::RoleState;
use self::team::Teams;
use self::verdict::Verdict;

pub struct Game {
    pub settings : Settings,

    pub players: Box<[Player]>,
    pub graves: Vec<Grave>,
    pub teams: Teams,

    phase_machine : PhaseStateMachine,
}

impl Game {
    pub fn new(settings: Settings, lobby_players: Vec<LobbyPlayer>) -> Self{
        let mut roles = create_random_roles(&settings.excluded_roles, &settings.role_list);
        roles.shuffle(&mut thread_rng());

        let mut players = Vec::new();
        for (player_index, player) in lobby_players.iter().enumerate() {
            let new_player = Player::new(
                player.name.clone(),
                player.sender.clone(),
                match roles.get(player_index){
                    Some(role) => *role,
                    None => RoleOutline::Any.get_random_role(&settings.excluded_roles, &roles),
                }
            );
            players.push(new_player);
        }
        drop(roles); // Ensure we don't use the order of roles anywhere

        let mut game = Self{
            players: players.into_boxed_slice(),
            graves: Vec::new(),
            teams: Teams::default(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings,
        };

        for player_ref in PlayerReference::all_players(&game){
            player_ref.send_join_game_data(&mut game);
        }

        for player_ref in PlayerReference::all_players(&game){
            let role_data_copy = player_ref.role_state(&game).clone();
            player_ref.set_role(&mut game, role_data_copy);
        }

        Teams::on_team_creation(&mut game);

        game
    }

    /// Returns a tuple containing the number of guilty votes and the number of innocent votes
    pub fn count_verdict_votes(&self, player_on_trial: PlayerReference)->(u8,u8){
        let mut guilty = 0;
        let mut innocent = 0;
        for player_ref in PlayerReference::all_players(self){
            if !player_ref.alive(self) || player_ref == player_on_trial {
                continue;
            }
            let mut voting_power = 1;
            if let RoleState::Mayor(mayor) = player_ref.role_state(self).clone(){
                if mayor.revealed {
                    voting_power += 2;
                }
            }
            
            match player_ref.verdict(self) {
                Verdict::Innocent => innocent += voting_power,
                Verdict::Abstain => {},
                Verdict::Guilty => guilty += voting_power,
            }
        }
        (guilty, innocent)
    }

    pub fn current_phase(&self) -> &PhaseState {
        &self.phase_machine.current_state
    }

    pub fn day_number(&self) -> u8 {
        self.phase_machine.day_number
    }

    pub fn tick(&mut self, time_passed: Duration){
        if self.phase_machine.day_number == u8::MAX {
            self.send_packet_to_all(ToClientPacket::GameOver{ reason: GameOverReason::ReachedMaxDay });
            // TODO, clean up the lobby. Stop the ticking
            return;
        }

        while self.phase_machine.time_remaining <= Duration::ZERO {
            let new_phase = PhaseState::end(self);

            self.start_phase(new_phase);
        }

        for player_ref in PlayerReference::all_players(self){
            player_ref.tick(self, time_passed)
        }
        
        self.phase_machine.time_remaining = self.phase_machine.time_remaining.saturating_sub(time_passed);
    }

    pub fn start_phase(&mut self, phase: PhaseState){
        self.phase_machine.current_state = phase;
        self.phase_machine.time_remaining = self.settings.phase_times.get_time_for(self.current_phase().phase());

        if PlayerReference::all_players(self).into_iter().filter(|p|p.alive(self)).collect::<Vec<_>>().len() <= 3{
            self.phase_machine.time_remaining /= 2
        }

        PhaseState::start(self);

        for player_ref in PlayerReference::all_players(self){
            player_ref.on_phase_start(self, self.current_phase().phase());
        }

        Teams::on_phase_start(self);

        self.send_packet_to_all(ToClientPacket::Phase { 
            phase: self.current_phase().phase(),
            day_number: self.phase_machine.day_number,
            seconds_left: self.phase_machine.time_remaining.as_secs()
        });
    }

    pub fn add_message_to_chat_group(&mut self, group: ChatGroup, mut message: ChatMessage){
        if let ChatMessage::Normal { chat_group, .. } = &mut message {
            *chat_group = group.clone();
        }

        for player_ref in group.all_players_in_group(self){
            player_ref.add_chat_message(self, message.clone());
            player_ref.send_chat_messages(self);
        }
    }
    pub fn add_messages_to_chat_group(&mut self, group: ChatGroup, messages: Vec<ChatMessage>){
        for message in messages.into_iter(){
            self.add_message_to_chat_group(group.clone(), message);
        }
    }

    pub fn send_packet_to_all(&mut self, packet: ToClientPacket){
        for player_ref in PlayerReference::all_players(self){
            player_ref.send_packet(self, packet.clone());
        }
    }

}

pub mod test {
    use rand::{thread_rng, seq::SliceRandom};

    use super::{Game, settings::Settings, role_list::{create_random_roles, RoleOutline}, player::{PlayerReference, test::mock_player}, phase::PhaseStateMachine, team::Teams};

    pub fn mock_game(settings: Settings, number_of_players: usize) -> Game {
        let mut roles = create_random_roles(&settings.excluded_roles, &settings.role_list);
        roles.shuffle(&mut thread_rng());
        
        let mut players = Vec::new();
        for player_index in 0..number_of_players {
            let new_player = mock_player(
                format!("{}",player_index),
                match roles.get(player_index){
                    Some(role) => *role,
                    None => RoleOutline::Any.get_random_role(&settings.excluded_roles, &roles),
                }
            );
            players.push(new_player);
        }
        drop(roles);

        let mut game = Game{
            players: players.into_boxed_slice(),
            graves: Vec::new(),
            teams: Teams::default(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings,
        };

        for player_ref in PlayerReference::all_players(&game){
            player_ref.send_join_game_data(&mut game);
        }

        for player_ref in PlayerReference::all_players(&game){
            let role_data_copy = player_ref.role_state(&game).clone();
            player_ref.set_role(&mut game, role_data_copy);
        }

        Teams::on_team_creation(&mut game);
        
        game
    }
}