

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use lazy_static::lazy_static;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::lobby::LobbyPlayer;
use crate::network::packet::{ToServerPacket, ToClientPacket, self, PlayerButtons};
use crate::prelude::*;
use super::chat::{ChatMessage, ChatGroup};
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
            players.push(
                Player::new(
                    player_index as u8,
                    lobby_players[player_index].name.clone(),
                    lobby_players[player_index].sender.clone(),
                    super::role::Role::Sheriff
                )
            );  //TODO role
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
        game.send_to_all(ToClientPacket::Phase { 
            phase: game.get_current_phase(), 
            seconds_left: game.phase_machine.time_remaining.as_secs(), 
            day_number: game.phase_machine.day_number 
        });
            
        for player in game.players.iter(){
            player.send(ToClientPacket::YourPlayerIndex { player_index: player.index });
            player.send(ToClientPacket::PlayerButtons { buttons: 
                PlayerButtons::from(&game, player.index)
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
        
        //Stuff that runs every tick
        for player in self.players.iter_mut(){
            player.tick()
        }

        if self.phase_machine.time_remaining <= Duration::ZERO{
            let new_phase = PhaseType::end(self);
            //reset
            for player in self.players.iter_mut(){
                player.reset_phase_variables(new_phase);
            }
            self.reset(new_phase);
            //start next phase
            self.jump_to_phase(new_phase);  //phase start is called here
        }
        
        self.phase_machine.time_remaining = match self.phase_machine.time_remaining.checked_sub(time_passed){
            Some(out) => out,
            None => Duration::ZERO,
        };
    }
    fn jump_to_phase(&mut self, phase: PhaseType){
        self.phase_machine.current_state = phase;
        //fix time
        self.phase_machine.time_remaining += self.phase_machine.current_state.get_length(&self.settings.phase_times);
        //call start
        PhaseType::start(self);

        //stuff that runs only when phase switches
        let mut alive = Vec::new();
        //stuff that runs only when phase switches
        for player in self.players.iter(){
            player.send(ToClientPacket::PlayerButtons{
                buttons: PlayerButtons::from(self, player.index) 
            });
            alive.push(player.alive);
        }
        self.send_to_all(ToClientPacket::Phase { phase: self.get_current_phase(), day_number: self.phase_machine.day_number, seconds_left: self.phase_machine.time_remaining.as_secs() });
        self.send_to_all(ToClientPacket::PlayerAlive { alive });
    }

    pub fn add_message_to_chat_group(&mut self, group: ChatGroup, message: ChatMessage){
        let mut message = message.clone();
        if let ChatMessage::Normal { message_sender, text, chat_group } = &mut message {
            *chat_group = group.clone();
        }
        for i in 0..self.players.len(){
            let player = self.get_unchecked_player(i as u8);
            let role = player.get_role();

            role.get_current_chat_groups(player.index, self).contains(&group);
            let player = self.get_unchecked_mut_player(i as u8);
            player.add_chat_message(message.clone());
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

    pub fn on_client_message(&mut self, player_index: PlayerIndex, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::Vote { player_index: player_voted_index } => {

                //Set vote
                let player = self.get_unchecked_mut_player(player_index);
                player.voting_variables.chosen_vote = player_voted_index;

                player.send(ToClientPacket::YourVoting { player_index: player_voted_index });
                let chat_message = ChatMessage::Voted { voter: player.index, votee: player_voted_index };
                self.add_message_to_chat_group(ChatGroup::All, chat_message);


                //get all votes on people
                let mut living_players_count = 0;
                let mut voted_for_player: Vec<u8> = Vec::new();

                for _ in self.players.iter(){
                    voted_for_player.push(0);
                }

                for player in self.players.iter(){
                    if player.alive{
                        living_players_count+=1;

                        if let Some(player_voted) = player.voting_variables.chosen_vote{
                            if let Some(num_votes) = voted_for_player.get_mut(player_voted as usize){
                                *num_votes+=1;
                            }
                        }
                    }
                }

                //if someone was voted
                let mut player_voted = None;
                for player_index in 0..voted_for_player.len(){
                    let num_votes = voted_for_player[player_index];
                    if num_votes > (living_players_count / 2){
                        player_voted = Some(player_index as u8);
                        break;
                    }
                }
                
                self.send_to_all(ToClientPacket::PlayerVotes { voted_for_player });

                if let Some(player_voted_index) = player_voted{
                    self.player_on_trial = player_voted;

                    self.send_to_all(ToClientPacket::PlayerOnTrial { player_index: player_voted_index } );
                    self.jump_to_phase(PhaseType::Judgement);
                }
            },
            ToServerPacket::Judgement { verdict } => {},
            ToServerPacket::Target { player_index_list } => {
                let player = self.get_unchecked_mut_player(player_index);
                player.night_variables.chosen_targets = player_index_list.clone();
                player.send(ToClientPacket::YourTarget { player_indices: player_index_list });
            },
            ToServerPacket::DayTarget { player_index } => {
            },
            ToServerPacket::SendMessage { text } => {},
            ToServerPacket::SendWhisper { player_index, text } => {},
            ToServerPacket::SaveWill { will } => {},
            _ => unreachable!()
        }
    }
    pub fn send_to_all(&self, packet: ToClientPacket){
        for player in self.players.iter(){
            player.send(packet.clone());
        }
    }

}