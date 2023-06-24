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
    pub fn new(settings: Settings, lobby_players: Vec<LobbyPlayer>)->Self{

        //create role list
        let mut roles = create_random_roles(&settings.excluded_roles, &settings.role_list);
        roles.shuffle(&mut thread_rng());
        

        //create players
        let mut players = Vec::new();
        for (player_index, player) in lobby_players.iter().enumerate() {
            let new_player = Player::new(
                player.name.clone(),
                player.sender.clone(),
                match roles.get(player_index){
                    Some(role) => *role,
                    None => RoleListEntry::Any.get_random_role(&settings.excluded_roles, &roles),
                }
            );
            players.push(new_player);
        }
        drop(roles);
        //just to make sure the order of roles is not used anywhere else for secuity from our own stupidity  
        let mut game = Self{
            players: players.into_boxed_slice(),
            graves: Vec::new(),
            teams: Teams::default(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings,
        };



        //set up role data
        for player_ref in PlayerReference::all_players(&game){
            let role_data_copy = player_ref.role_state(&game).clone();
            player_ref.set_role(&mut game, role_data_copy);
        }

        Teams::on_team_creation(&mut game);

        for player_ref in PlayerReference::all_players(&game){
            game.send_join_game_information(player_ref)
        }
        game
    }

    pub fn send_join_game_information(&mut self, player_ref: PlayerReference){


        //GENERAL GAME
        player_ref.send_packets(self, vec![
            ToClientPacket::Players{ 
                names: PlayerReference::all_players(self).iter().map(|p|{return p.name(self).clone()}).collect()
            },
            ToClientPacket::RoleList {role_list: self.settings.role_list.clone()},
            ToClientPacket::Phase { 
                phase: self.current_phase().phase(),
                seconds_left: self.phase_machine.time_remaining.as_secs(), 
                day_number: self.phase_machine.day_number 
            },
            ToClientPacket::PlayerAlive{
                alive: PlayerReference::all_players(self).into_iter().map(|p|p.alive(self)).collect()
            }
        ]);

        if let PhaseState::Testimony { player_on_trial, .. }
            | PhaseState::Judgement { player_on_trial, .. }
            | PhaseState::Evening { player_on_trial: Some(player_on_trial) } = self.current_phase() {
            player_ref.send_packet(self, ToClientPacket::PlayerOnTrial{
                player_index: player_on_trial.index()
            });
        }
        let votes_packet = ToClientPacket::new_player_votes(self);
        player_ref.send_packet(self, votes_packet);
        for grave in self.graves.iter(){
            player_ref.send_packet(self, ToClientPacket::AddGrave { grave: grave.clone() });
        }



        //PLAYER SPECIFIC

        player_ref.send_packets(self, vec![
            ToClientPacket::YourName{
                name: player_ref.name(self).clone()
            },
            ToClientPacket::YourPlayerIndex { 
                player_index: player_ref.index() 
            },
            ToClientPacket::YourRoleState {
                role_state: player_ref.role_state(self).clone()
            },
            ToClientPacket::YourRoleLabels { 
                role_labels: PlayerReference::ref_map_to_index(player_ref.role_labels(self).clone()) 
            },
            ToClientPacket::YourPlayerTags { 
                player_tags: PlayerReference::ref_map_to_index(player_ref.player_tags(self).clone())
            },
            ToClientPacket::YourTarget{
                player_indices: PlayerReference::ref_vec_to_index(player_ref.chosen_targets(self))
            },
            ToClientPacket::YourJudgement{
                verdict: player_ref.verdict(self)
            },
            ToClientPacket::YourVoting{ 
                player_index: PlayerReference::ref_option_to_index(&player_ref.chosen_vote(self))
            },
            ToClientPacket::YourWill{
                will: player_ref.will(self).clone()
            },
            ToClientPacket::YourNotes{
                notes: player_ref.notes(self).clone()
            }
        ]);
        

        let buttons = AvailableButtons::from_player(self, player_ref);
        player_ref.send_packet(self, ToClientPacket::YourButtons{buttons});
    }

    //returns (guilty, innocent)
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

    //phase state machine
    pub fn tick(&mut self, time_passed: Duration){
        
        //if max day is reached, end game
        if self.phase_machine.day_number == u8::MAX {
            self.send_packet_to_all(ToClientPacket::GameOver{ reason: GameOverReason::ReachedMaxDay });
            // TODO, clean up the lobby. Stop the ticking
            return;
        }

        //check if phase is over and start next phase
        while self.phase_machine.time_remaining <= Duration::ZERO {

            let new_phase = PhaseState::end(self);

            self.start_phase(new_phase);
        }

        for player_ref in PlayerReference::all_players(self){
            player_ref.tick(self, time_passed)
        }
        
        //subtract time for actual tick
        self.phase_machine.time_remaining = self.phase_machine.time_remaining.saturating_sub(time_passed);
    }

    pub fn start_phase(&mut self, phase: PhaseState){

        self.phase_machine.current_state = phase;
        self.phase_machine.time_remaining = self.settings.phase_times.get_time_for(self.current_phase().phase());

        PhaseState::start(self); //THIS WAS RECENTLY MOVED BEFORE THE ON_PHASE_STARTS, PRAY THAT IT WONT CAUSE PROBLEMS

        //player reset
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

pub mod test {
    use rand::{thread_rng, seq::SliceRandom};

    use super::{Game, settings::Settings, role_list::{create_random_roles, RoleListEntry}, player::{PlayerReference, test::mock_player}, phase::PhaseStateMachine, team::Teams};

    pub fn mock_game(settings: Settings, number_of_players: usize) -> Game {
        //create role list
        let mut roles = create_random_roles(&settings.excluded_roles, &settings.role_list);
        roles.shuffle(&mut thread_rng());
        
        //create players
        let mut players = Vec::new();
        for player_index in 0..number_of_players {
            let new_player = mock_player(
                player_index.to_string(),
                match roles.get(player_index){
                    Some(role) => *role,
                    None => RoleListEntry::Any.get_random_role(&settings.excluded_roles, &roles),
                }
            );
            players.push(new_player);
        }
        drop(roles);
        //just to make sure the order of roles is not used anywhere else for secuity from our own stupidity  
        let mut game = Game {
            players: players.into_boxed_slice(),
            graves: Vec::new(),
            teams: Teams::default(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings,
        };



        //set up role data
        for player_ref in PlayerReference::all_players(&game){
            let role_data_copy = player_ref.role_state(&game).clone();
            player_ref.set_role(&mut game, role_data_copy);
        }
        
        Teams::on_team_creation(&mut game);

        for player_ref in PlayerReference::all_players(&game){
            game.send_join_game_information(player_ref)
        }
        game
    }
}