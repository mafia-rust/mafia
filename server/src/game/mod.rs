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

use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::lobby::LobbyPlayer;
use crate::packet::{ToClientPacket, GameOverReason};
use available_buttons::AvailableButtons;
use chat::{ChatMessage, ChatGroup};
use player::PlayerReference;
use role_list::{RoleListEntry, create_random_roles};
use player::Player;
use phase::{PhaseStateMachine, PhaseType};
use settings::Settings;
use grave::Grave;

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
        let mut roles = create_random_roles(&settings.role_list);
        roles.shuffle(&mut thread_rng());
        

        //create players
        let mut players = Vec::new();
        for (player_index, player) in lobby_players.iter().enumerate() {
            let new_player = Player::new(
                player.name.clone(),
                player.sender.clone(),
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



        //set up role data
        for player_ref in PlayerReference::all_players(&game){
            let role_data_copy = player_ref.role_data(&game).clone();
            player_ref.set_role(&mut game, role_data_copy);
        }

        
        for player_ref in PlayerReference::all_players(&game){
            game.send_start_game_information(player_ref)
        }
        game
    }

    pub fn send_start_game_information(&mut self, player_ref: PlayerReference){
        player_ref.send_packet(self, ToClientPacket::YourRoleData { role_data: player_ref.role_data(self).clone() });
        player_ref.send_packet(self, 
            ToClientPacket::Players{ 
                names: PlayerReference::all_players(&self).iter().map(|p|{return p.name(&self).clone()}).collect()
            }
        );

        player_ref.send_packet(self, ToClientPacket::YourPlayerIndex { player_index: player_ref.index() });
        player_ref.send_packet(self, ToClientPacket::RoleList {role_list: self.settings.role_list.clone()});
        player_ref.send_packet(self, ToClientPacket::Phase { 
            phase: self.current_phase(),
            seconds_left: self.phase_machine.time_remaining.as_secs(), 
            day_number: self.phase_machine.day_number 
        });
        
        player_ref.send_packet(self, ToClientPacket::YourRoleLabels { role_labels: PlayerReference::ref_map_to_index(player_ref.role_labels(self).clone()) });

        let buttons = AvailableButtons::from_player(&self, player_ref);
        player_ref.send_packet(self, ToClientPacket::YourButtons{buttons});
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
        while self.phase_machine.time_remaining <= Duration::ZERO {

            let new_phase = PhaseType::end(self);

            self.start_phase(new_phase);
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

    pub fn on_phase_start(&mut self, phase: PhaseType){
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
    pub fn start_phase(&mut self, phase: PhaseType){

        self.phase_machine.current_state = phase;
        self.phase_machine.time_remaining = self.phase_machine.current_state.get_length(&self.settings.phase_times);

        PhaseType::start(self); //THIS WAS RECENTLY MOVED BEFORE THE ON_PHASE_STARTS, PRAY THAT IT WONT CAUSE PROBLEMS


        //player reset
        for player_ref in PlayerReference::all_players(self){
            player_ref.on_phase_start(self, phase);
            player_ref.role(self).on_phase_start(self, player_ref, phase);
        }

        //game reset
        self.on_phase_start(phase);



        self.send_packet_to_all(ToClientPacket::Phase { 
            phase,
            day_number: self.phase_machine.day_number,
            seconds_left: self.phase_machine.time_remaining.as_secs()
        });
    }

    pub fn add_message_to_chat_group(&mut self, group: ChatGroup, mut message: ChatMessage){
        //if normal message, then correct chat group
        if let ChatMessage::Normal { chat_group, .. } = &mut message {
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

    pub fn send_packet_to_all(&mut self, packet: ToClientPacket){
        for player_ref in PlayerReference::all_players(self){
            player_ref.send_packet(self, packet.clone());
        }
    }

}