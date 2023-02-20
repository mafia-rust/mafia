use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::network::packet::{ToServerPacket, ToClientPacket};
use crate::prelude::*;
use super::grave::Grave;
use super::phase::{Phase, PhaseStateMachine};
use super::player::{Player, PlayerIndex};

pub struct Game {
    pub players: Vec<Player>,   // PlayerIndex is the index into this vec, should be unchanging as game goes on
    pub graves: Vec<Grave>,

    // pub role_list: Vec<Role>,
    // pub invesigator_results: TODO

    pub phase_machine : PhaseStateMachine,
}

impl Game {
    pub fn new(players_sender_and_name: Vec<(UnboundedSender<ToClientPacket>, String)>)->Self{

        let mut players = Vec::new();

        //create players
        for player_index in 0..players_sender_and_name.len(){
            let (sender, name) = players_sender_and_name.get(player_index).expect("index should exist because for loop");
            players.push(Player::new(player_index, name.clone(), sender.clone(), super::role::Role::Sheriff));  //TODO sheriff!
        }

        //send to players all game information stuff

        Self{
            players,
            graves: Vec::new(),
            phase_machine: PhaseStateMachine::new(),
        }
    }
    pub fn get_player(&self, index: PlayerIndex) -> Result<&Player> {
        self.players.get(index).ok_or_else(|| err!(generic, "Failed to get player {}", index))
    }

    pub fn get_player_mut(&mut self, index: PlayerIndex) -> Result<&mut Player> {
        self.players.get_mut(index).ok_or_else(|| err!(generic, "Failed to get player {}", index))
    }

    pub fn get_current_phase(&self) -> Phase {
        self.phase_machine.current_state
    }

    pub fn on_client_message(&mut self, player_index: PlayerIndex, incoming_packet : ToServerPacket){

    }
}