

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use lazy_static::lazy_static;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::lobby::LobbyPlayer;
use crate::network::packet::{ToServerPacket, ToClientPacket, self, PlayerButtons};
use crate::prelude::*;
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

        let mut players = Vec::new();

        //create players
        for player_index in 0..lobby_players.len(){
            players.push(Player::new(player_index as u8, lobby_players[player_index].name.clone(), lobby_players[player_index].sender.clone(), super::role::Role::Consort));  //TODO role
        }

        let game = Self{
            players,
            graves: Vec::new(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings,

            player_on_trial: None,
            trials_left: 0,
        };

        //send to players all game information stuff
        let player_names: Vec<String> = game.players.iter().map(|p|{return p.name.clone()}).collect();
        game.send_to_all(ToClientPacket::Players { names: player_names });
        for player in game.players.iter(){
            player.send(ToClientPacket::PlayerButtons { buttons: 
                PlayerButtons::from(&game, player.index)
            });
        }

        //start clock TODO
        //call phase tick stuff
        // tokio::spawn(||{
        //     game.phase_machine
        // })
        
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

    pub fn reset(&mut self, phase: PhaseType){
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

    pub fn tick(&mut self, time_passed: Duration){
        self.phase_machine.tick(self, time_passed);
    }

    pub fn on_client_message(&mut self, player_index: PlayerIndex, incoming_packet : ToServerPacket){

    }
    pub fn send_to_all(&self, packet: ToClientPacket){
        for player in self.players.iter(){
            player.send(packet.clone());
        }
    }

    
    
}