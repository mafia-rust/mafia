

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
use super::chat::night_message::NightInformation;
use super::chat::{ChatMessage, ChatGroup, MessageSender};
use super::grave::{GraveRole, GraveKiller};
use super::role_list::RoleListEntry;
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
        let roles = settings.role_list.create_random_roles();
        

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
            new_player.set_role(new_player.role_data);
            players.push(new_player);
        }

        let game = Self{
            players,
            graves: Vec::new(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings: settings.clone(),

            player_on_trial: None,
            trials_left: 0,
        };

        //send to players all game information stuff
        let player_names: Vec<String> = game.players.iter().map(|p|{return p.name.clone()}).collect();
        game.send_to_all(ToClientPacket::Players { names: player_names });
        game.send_to_all(ToClientPacket::RoleList { 
            role_list: settings.role_list.clone() 
        });
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

        while self.phase_machine.time_remaining <= Duration::ZERO{
            let new_phase = PhaseType::end(self);
            //reset
            for player_index in 0..self.players.len(){
                Player::reset_phase_variables(self, player_index as PlayerIndex, new_phase);
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

    pub fn on_client_message(&mut self, player_index: PlayerIndex, incoming_packet: ToServerPacket){
        match incoming_packet {
            ToServerPacket::Vote { player_index: mut player_voted_index } => {

                if self.phase_machine.current_state != PhaseType::Voting || (player_voted_index.is_some() && self.players.len() <= player_voted_index.unwrap() as usize){
                    return;
                }

                //Set vote
                let player = self.get_unchecked_mut_player(player_index);

                //if player being voted for is dead then no
                if !player.alive { player_voted_index = None; }
                
                player.send(ToClientPacket::YourVoting { player_index: player_voted_index });

                if player.voting_variables.chosen_vote == player_voted_index {
                    return;
                }
                
                player.voting_variables.chosen_vote = player_voted_index;

                
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
                    self.jump_to_phase(PhaseType::Testimony);
                }
            },
            ToServerPacket::Judgement { verdict } => {
                if self.phase_machine.current_state != PhaseType::Judgement{
                    return;
                }

                let player = self.get_unchecked_mut_player(player_index);
                
                player.send(ToClientPacket::YourJudgement { verdict: verdict.clone() });
                if player.voting_variables.verdict == verdict {
                    return;
                }
                player.voting_variables.verdict = verdict.clone();
                self.add_message_to_chat_group(ChatGroup::All, ChatMessage::JudgementVote { voter_player_index: player_index });
            },
            ToServerPacket::Target { player_index_list } => {
                //TODO can target????
                //TODO Send you targeted someone message in correct chat.
                if self.phase_machine.current_state != PhaseType::Night{
                    return;
                }

                self.get_unchecked_mut_player(player_index).night_variables.chosen_targets = vec![];
                let role = self.get_unchecked_mut_player(player_index).get_role();

                for target_index in player_index_list {
                    if role.can_night_target(player_index, target_index, self) {
                        self.get_unchecked_mut_player(player_index).night_variables.chosen_targets.push(target_index);
                    }
                }

                let player = self.get_unchecked_mut_player(player_index);

                player.send(ToClientPacket::YourTarget { player_indices: player.night_variables.chosen_targets.clone() });
            },
            ToServerPacket::DayTarget { player_index } => {
                //TODO can daytarget???
                //TODO
            },
            ToServerPacket::SendMessage { text } => {
                let player = self.get_unchecked_mut_player(player_index);
                
                for chat_group in player.get_role().get_current_chat_groups(player_index, self){
                    self.add_message_to_chat_group(
                        chat_group.clone(),
                        //TODO message sender, Jailor & medium
                        ChatMessage::Normal { message_sender: MessageSender::Player(player_index) , text: text.clone(), chat_group }
                    );
                }
            },
            ToServerPacket::SendWhisper { player_index: whispered_to_player_index, text } => {

                //ensure its day and your not whispering yourself and the other player exists
                if !self.get_current_phase().is_day() || whispered_to_player_index == player_index || self.players.len() <= whispered_to_player_index as usize{
                    return;
                }

                self.add_message_to_chat_group(ChatGroup::All, ChatMessage::BroadcastWhisper { whisperer: player_index, whisperee: whispered_to_player_index });
                let message = ChatMessage::Whisper { 
                    from_player_index: player_index, 
                    to_player_index: whispered_to_player_index, 
                    text 
                };
        
                let to_player = self.get_unchecked_mut_player(whispered_to_player_index);
                to_player.add_chat_message(message.clone());

                let from_player = self.get_unchecked_mut_player(player_index);
                from_player.add_chat_message(message);
                

                //TODO, send to blackmailer
            },
            ToServerPacket::SaveWill { will } => {
                let player = self.get_unchecked_mut_player(player_index);
                player.will = will.clone();
                player.send(ToClientPacket::YourWill { will });
            },
            _ => unreachable!()
        }
        
        let packet = ToClientPacket::PlayerButtons { buttons: PlayerButtons::from(self, player_index)};
        self.get_unchecked_mut_player(player_index).send(packet);

        for player in self.players.iter_mut(){
            player.send_chat_messages();
        }

    }
    pub fn send_to_all(&self, packet: ToClientPacket){
        for player in self.players.iter(){
            player.send(packet.clone());
        }
    }

}