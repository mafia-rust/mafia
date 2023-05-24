

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
use crate::packet::{ToClientPacket, GameOverReason};
use crate::prelude::*;
use super::available_buttons::AvailableButtons;
use super::chat::night_message::NightInformation;
use super::chat::{ChatMessage, ChatGroup, MessageSender};
use super::grave::{GraveRole, GraveKiller};
use super::player::PlayerReference;
use super::role_list::{RoleListEntry, create_random_roles};
use super::settings;
use super::player::Player;
use super::phase::{PhaseStateMachine, PhaseType};
use super::role_list::RoleList;
use super::settings::Settings;
use super::grave::Grave;

pub struct Game {
    pub settings : Settings,

    pub players: Box<[Player]>,
    pub graves: Vec<Grave>,

    pub phase_machine : PhaseStateMachine,

    pub player_on_trial: Option<PlayerReference>,   //resets on morning
    pub trials_left: u8,    //resets on morning
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
                    None => RoleListEntry::Any.get_random_role(&roles),
                }
            );
            players.push(new_player);
        }
        drop(roles);
        //just to make sure the order of roles is not used anywhere else for secuity from our own stupidity  
        let mut game = Self{
            players: players.into_boxed_slice(),
            graves: Vec::new(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings: settings.clone(),

            player_on_trial: None,
            trials_left: 0,
        };

        game.send_packet_to_all(ToClientPacket::Players { 
            names: PlayerReference::all_players(&game).iter().map(|p|{return p.name(&game).clone()}).collect() 
        });

        //set up role data
        for player_ref in PlayerReference::all_players(&game){
            let role_data_copy = player_ref.role_data(&game).clone();
            player_ref.set_role(&mut game, role_data_copy);
            player_ref.send_packet(&game, ToClientPacket::YourPlayerIndex { player_index: *player_ref.index() })
        }

        //send to players all game information stuff

        
        game.send_packet_to_all(ToClientPacket::RoleList { 
            role_list: settings.role_list.clone() 
        });
        game.send_packet_to_all(ToClientPacket::Phase { 
            phase: game.current_phase(),
            seconds_left: game.phase_machine.time_remaining.as_secs(), 
            day_number: game.phase_machine.day_number 
        });
            
        for player_ref in PlayerReference::all_players(&game){
            player_ref.send_packet(&game, ToClientPacket::YourButtons { buttons: 
                AvailableButtons::from_player(&game, player_ref)
            });
        }
        
        game
    }

    pub fn current_phase(&self) -> PhaseType {
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

        //check if phase is over and start next phase
        while self.phase_machine.time_remaining <= Duration::ZERO{
            let new_phase = PhaseType::end(self);

            //player reset
            for player_ref in PlayerReference::all_players(self){
                player_ref.reset_phase_start(self, new_phase);
                player_ref.role(&self).on_phase_start(self, player_ref, new_phase);
            }

            //game reset
            self.reset_phase_start(new_phase);
            
            //phase start
            self.jump_to_start_phase(new_phase);
            
        }

        for player_ref in PlayerReference::all_players(self){
            player_ref.tick(self)
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

        self.send_packet_to_all(ToClientPacket::Phase { 
            phase: phase, 
            day_number: self.phase_machine.day_number, 
            seconds_left: self.phase_machine.time_remaining.as_secs() 
        });
    }

    pub fn add_message_to_chat_group(&mut self, group: ChatGroup, message: ChatMessage){
        let mut message = message.clone();

        //if normal message, then correct chat group
        if let ChatMessage::Normal { message_sender, text, chat_group } = &mut message {
            *chat_group = group.clone();
        }

        //add messages
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

    pub fn send_packet_to_all(&self, packet: ToClientPacket){
        for player_ref in PlayerReference::all_players(self){
            player_ref.send_packet(self, packet.clone());
        }
    }

}