

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::lobby::LobbyPlayer;
use crate::network::packet::{ToServerPacket, ToClientPacket, self, YourButtons, GameOverReason};
use crate::prelude::*;
use super::chat::night_message::NightInformation;
use super::chat::{ChatMessage, ChatGroup, MessageSender};
use super::grave::{GraveRole, GraveKiller};
use super::role_list::{RoleListEntry, create_random_roles};
use super::settings;
use super::{phase::{PhaseStateMachine, PhaseType}, player::{Player, PlayerIndex}, role_list::RoleList, settings::Settings, grave::Grave};

pub struct Game {
    pub settings : Settings,

    pub players: Vec<Player>,   // PlayerIndex is the index into this vec, should be unchanging as game goes on
    pub graves: Vec<Grave>,

    pub phase_machine : PhaseStateMachine,

    pub player_on_trial: Option<PlayerIndex>,   //Morning
    pub trials_left: u8,                //Morning
}

impl Game {
    pub fn new(settings: Settings, lobby_players: Vec<LobbyPlayer>)->Self{

        //create role list
        let mut settings = settings.clone();
        let mut roles = create_random_roles(&settings.role_list);
        roles.shuffle(&mut thread_rng());
        

        //create players
        let mut players = Vec::new();
        for player_index in 0..lobby_players.len(){
            let mut new_player = Player::new(         
                player_index as u8,
                lobby_players[player_index].name.clone(),
                lobby_players[player_index].sender.clone(),
                match roles.get(player_index){
                    Some(role) => *role,
                    None => RoleListEntry::Any.get_random_role(),
                }
            );
            players.push(new_player);
        }
        drop(roles);
        //just to make sure the order of roles is not used anywhere else for secuity from our own stupidity  

        let mut game = Self{
            players,
            graves: Vec::new(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings: settings.clone(),

            player_on_trial: None,
            trials_left: 0,
        };

        //set up role data
        for player_index in 0..(game.players.len() as PlayerIndex){
            let role_data_copy = game.get_unchecked_mut_player(player_index).role_data().clone();
            Player::set_role(&mut game, player_index, role_data_copy);
        }

        //send to players all game information stuff
        let player_names: Vec<String> = game.players.iter().map(|p|{return p.name().clone()}).collect();
        game.send_packet_to_all(ToClientPacket::Players { names: player_names });
        game.send_packet_to_all(ToClientPacket::RoleList { 
            role_list: settings.role_list.clone() 
        });
        game.send_packet_to_all(ToClientPacket::Phase { 
            phase: game.get_current_phase(), 
            seconds_left: game.phase_machine.time_remaining.as_secs(), 
            day_number: game.phase_machine.day_number 
        });
            
        for player in game.players.iter(){
            player.send_packet(ToClientPacket::YourButtons { buttons: 
                YourButtons::from(&game, player.index().clone())
            });
        }
        
        game
    }

    pub fn get_player(&self, index: PlayerIndex)->Option<&Player>{
        self.players.get(index as usize)
    }
    pub fn get_player_mut(&mut self, index: PlayerIndex)->Option<&mut Player>{
        self.players.get_mut(index as usize)
    }
    pub fn get_unchecked_player(&self, index: PlayerIndex)->&Player{
        self.players.get(index as usize).unwrap()
    }
    pub fn get_unchecked_mut_player(&mut self, index: PlayerIndex)->&mut Player{
        self.players.get_mut(index as usize).unwrap()
    }

    pub fn get_current_phase(&self) -> PhaseType {
        self.phase_machine.current_state
    }

    //phase state machine
    pub fn tick(&mut self, time_passed: Duration){
        
        //if max day is reached, end game
        if self.phase_machine.day_number == 255 {
            self.send_packet_to_all(ToClientPacket::GameOver{ reason: GameOverReason::ReachedMaxDay });
            // TODO, clean up the lobby. Stop the ticking
            return;
        }

        for player in self.players.iter_mut(){
            player.tick()
        }


        //check if phase is over and start next phase
        while self.phase_machine.time_remaining <= Duration::ZERO{
            let new_phase = PhaseType::end(self);

            //reset variables
            for player_index in 0..self.players.len(){
                Player::reset_phase_start(self, player_index as PlayerIndex, new_phase);
            }
            self.reset_phase_start(new_phase);
            
            self.jump_to_start_phase(new_phase);
        }
        
        //subtract time for actual tick
        self.phase_machine.time_remaining = match self.phase_machine.time_remaining.checked_sub(time_passed){
            Some(out) => out,
            None => Duration::ZERO,
        };
    }
    pub fn reset_phase_start(&mut self, phase: PhaseType){
        match phase {
            PhaseType::Morning => {
                self.player_on_trial = None;
                self.trials_left = 3;
            },
            PhaseType::Discussion => {},
            PhaseType::Voting => {},
            PhaseType::Testimony => {},
            PhaseType::Judgement => {},
            PhaseType::Evening => {},
            PhaseType::Night => {},
        }
    }
    pub fn jump_to_start_phase(&mut self, phase: PhaseType){
        self.phase_machine.current_state = phase;
        //fix time
        self.phase_machine.time_remaining += self.phase_machine.current_state.get_length(&self.settings.phase_times);
        //call start
        PhaseType::start(self);
    }

    pub fn add_message_to_chat_group(&mut self, group: ChatGroup, message: ChatMessage){
        let mut message = message.clone();

        //if normal message, then correct chat group
        if let ChatMessage::Normal { message_sender, text, chat_group } = &mut message {
            *chat_group = group.clone();
        }

        //add messages
        let players = group.all_players_in_group(self);
        for player_index in players.into_iter(){
            self.get_unchecked_mut_player(player_index).add_chat_message(message.clone());
        }

        //send messages to player
        for player in self.players.iter_mut(){
            player.send_chat_messages();
        }
    }
    pub fn add_messages_to_chat_group(&mut self, group: ChatGroup, messages: Vec<ChatMessage>){
        for message in messages.into_iter(){
            self.add_message_to_chat_group(group.clone(), message);
        }
    }

    pub fn send_packet_to_all(&self, packet: ToClientPacket){
        for player in self.players.iter(){
            player.send_packet(packet.clone());
        }
    }

}