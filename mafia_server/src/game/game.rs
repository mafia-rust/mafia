use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::network::packet::{ToServerPacket, ToClientPacket};
use crate::prelude::*;
use super::grave::Grave;
use super::phase::{PhaseStateMachine, PhaseType};
use super::player::{Player, PlayerIndex};
use super::role_list::RoleList;
use super::settings::Settings;

pub struct Game {
    pub settings : Settings,

    pub players: Vec<Player>,   // PlayerIndex is the index into this vec, should be unchanging as game goes on
    pub graves: Vec<Grave>,

    pub phase_machine : PhaseStateMachine,

    pub player_on_trial: Option<PlayerIndex>,   //Morning
    pub trials_left: u8,                //Morning
}

impl Game {
    pub fn new(settings: Settings, players_sender_and_name: Vec<(UnboundedSender<ToClientPacket>, String)>)->Self{

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
            phase_machine: PhaseStateMachine::new(settings.phase_times),
            settings,

            player_on_trial: None,
            trials_left: 0,
        }
    }
    pub fn get_player(&self, index: PlayerIndex) -> Result<&Player> {
        self.players.get(index).ok_or_else(|| err!(generic, "Failed to get player {}", index))
    }

    pub fn get_player_mut(&mut self, index: PlayerIndex) -> Result<&mut Player> {
        self.players.get_mut(index).ok_or_else(|| err!(generic, "Failed to get player {}", index))
    }

    pub fn get_current_phase(&self) -> PhaseType {
        self.phase_machine.current_state
    }

    pub fn on_client_message(&mut self, player_index: PlayerIndex, incoming_packet : ToServerPacket){

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
}