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
pub mod event;
pub mod spectator;

use std::collections::HashMap;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

use crate::client_connection::ClientConnection;
use crate::game::event::OnGameStart;
use crate::packet::ToClientPacket;
use chat::{ChatMessageVariant, ChatGroup, ChatMessage};
use player::PlayerReference;
use player::Player;
use phase::PhaseStateMachine;
use settings::Settings;
use grave::Grave;

use self::end_game_condition::EndGameCondition;
use self::event::{OnGameEnding, OnPhaseStart};
use self::phase::PhaseState;
use self::player::PlayerInitializeParameters;
use self::role::RoleState;
use self::spectator::Spectator;
use self::spectator::SpectatorInitializeParameters;
use self::team::Teams;
use self::verdict::Verdict;


pub struct Game {
    pub settings : Settings,

    pub spectators: Vec<Spectator>,
    pub spectator_chat_messages: Vec<ChatMessageVariant>,

    pub players: Box<[Player]>,
    pub graves: Vec<Grave>,
    pub teams: Teams,

    phase_machine : PhaseStateMachine,

    /// Whether the game is still updating phase times
    pub ticking: bool,
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RejectStartReason {
    GameEndsInstantly,
    RoleListTooSmall,
    RoleListCannotCreateRoles,
    ZeroTimeGame,
    PlayerDisconnected
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum GameOverReason {
    ReachedMaxDay,
    Winner,
    Draw
}



impl Game {
    pub fn new(settings: Settings, players: Vec<PlayerInitializeParameters>, spectators: Vec<SpectatorInitializeParameters>) -> Result<Self, RejectStartReason>{
        //check settings are not completly off the rails
        if settings.phase_times.game_ends_instantly() {
            return Err(RejectStartReason::ZeroTimeGame);
        }
        

        let mut role_generation_tries = 0;
        const MAX_ROLE_GENERATION_TRIES: u8 = 250;
        let mut game = loop {

            if role_generation_tries >= MAX_ROLE_GENERATION_TRIES {
                return Err(RejectStartReason::RoleListCannotCreateRoles);
            }

            let settings = settings.clone();
            let mut role_list = settings.role_list.clone();
            role_list.sort();


            let mut roles = match role_list.create_random_roles(&settings.excluded_roles){
                Some(roles) => {roles},
                None => {return Err(RejectStartReason::RoleListCannotCreateRoles);}
            };
            roles.shuffle(&mut thread_rng());






            let mut new_players = Vec::new();
            for (player_index, player) in players.iter().enumerate() {
                let ClientConnection::Connected(ref sender) = player.connection else {
                    return Err(RejectStartReason::PlayerDisconnected)
                };
                let new_player = Player::new(
                    player.name.clone(),
                    sender.clone(),
                    match roles.get(player_index){
                        Some(role) => *role,
                        None => {
                            return Err(RejectStartReason::RoleListTooSmall);
                        },
                    }
                );
                new_players.push(new_player);
            }
            drop(roles); // Ensure we don't use the order of roles anywhere

            let game = Self{
                ticking: true,
                spectators: spectators.clone().into_iter().map(|spectator|Spectator::new(spectator)).collect(),
                spectator_chat_messages: Vec::new(),
                players: new_players.into_boxed_slice(),
                graves: Vec::new(),
                teams: Teams::default(),
                phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
                settings,
            };

            if !game.game_is_over() {
                break game;
            }
            role_generation_tries += 1;
        };

        if game.game_is_over() {
            return Err(RejectStartReason::RoleListCannotCreateRoles);
        }


        OnGameStart::invoke(&mut game);

        Ok(game)
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
    pub fn count_votes_and_start_trial(&mut self){

        let &PhaseState::Nomination { trials_left } = self.current_phase() else {return};

        let mut living_players_count = 0;
        let mut voted_player_votes: HashMap<PlayerReference, u8> = HashMap::new();

        for player in PlayerReference::all_players(self){
            if !player.alive(self) { continue }
            living_players_count += 1;


            let Some(voted_player) = player.chosen_vote(self) else { continue };

            let mut voting_power = 1;
            if let RoleState::Mayor(mayor) = player.role_state(self).clone() {
                if mayor.revealed {
                    voting_power = 3;
                }
            }

            if let Some(num_votes) = voted_player_votes.get_mut(&voted_player) {
                *num_votes += voting_power;
            } else {
                voted_player_votes.insert(voted_player, voting_power);
            }
        }
        
        self.send_packet_to_all(
            ToClientPacket::PlayerVotes { votes_for_player: 
                PlayerReference::ref_map_to_index(voted_player_votes.clone())
            }
        );


        let mut next_player_on_trial = None;
        for (player, votes) in voted_player_votes.iter(){
            if *votes > (living_players_count / 2){
                next_player_on_trial = Some(*player);
                break;
            }
        }
        
        if let Some(player_on_trial) = next_player_on_trial {
            self.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: player_on_trial.index() } );
            self.start_phase(PhaseState::Testimony { trials_left: trials_left-1, player_on_trial });
        }        
    }

    pub fn game_is_over(&self) -> bool {
        //find list of all remaining teams, no duplicates, and remove none
        let remaining_teams: Vec<EndGameCondition> = 
            PlayerReference::all_players(self)
                .filter(|p|p.alive(self) && p.end_game_condition(self) != EndGameCondition::None)
                .map(|p|p.end_game_condition(self))
                .collect::<std::collections::HashSet<EndGameCondition>>().into_iter().collect::<Vec<EndGameCondition>>();

        //if there are no teams left and multiple amnesiacs alive then the game is not over
        if
            remaining_teams.is_empty() && 
            PlayerReference::all_players(self)
                .filter(|p|p.alive(self) && p.role_state(self).role() == role::Role::Amnesiac)
                .count() > 1 
        {
            return false;
        }
        
        remaining_teams.len() <= 1
    }

    pub fn current_phase(&self) -> &PhaseState {
        &self.phase_machine.current_state
    }

    pub fn day_number(&self) -> u8 {
        self.phase_machine.day_number
    }

    pub fn tick(&mut self, time_passed: Duration){

        if !self.ticking { return }

        if self.game_is_over() {
            OnGameEnding::invoke(self);
        }

        if self.phase_machine.day_number == u8::MAX {
            self.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::GameOver);
            self.send_packet_to_all(ToClientPacket::GameOver{ reason: GameOverReason::ReachedMaxDay });
            self.ticking = false;
            return;
        }

        while self.phase_machine.time_remaining <= Duration::ZERO {
            let new_phase = PhaseState::end(self);

            self.start_phase(new_phase);
        }
        PlayerReference::all_players(self).for_each(|p|p.tick(self, time_passed));

        self.phase_machine.time_remaining = self.phase_machine.time_remaining.saturating_sub(time_passed);
    }

    pub fn start_phase(&mut self, phase: PhaseState){
        self.phase_machine.current_state = phase;
        self.phase_machine.time_remaining = self.settings.phase_times.get_time_for(self.current_phase().phase());

        //if there are less than 3 players alive then the game is sped up by 2x
        if PlayerReference::all_players(self).filter(|p|p.alive(self)).count() <= 3{
            self.phase_machine.time_remaining /= 2;
        }

        PhaseState::start(self);
        OnPhaseStart::create_and_invoke(self, self.current_phase().phase());
    }

    pub fn add_grave(&mut self, grave: Grave){
        self.graves.push(grave.clone());
        event::OnGraveAdded::create_and_invoke(self, grave);
    }

    pub fn add_message_to_chat_group(&mut self, group: ChatGroup, variant: ChatMessageVariant){
        let message = ChatMessage::new_non_private(variant.clone(), group.clone());

        for player_ref in group.all_players_in_group(self){
            player_ref.add_chat_message(self, message.clone());
            player_ref.send_chat_messages(self);
        }

        if group == ChatGroup::All {
            self.add_chat_message_to_spectators(variant);
        }
    }
    pub fn add_messages_to_chat_group(&mut self, group: ChatGroup, messages: Vec<ChatMessageVariant>){
        for message in messages.into_iter(){
            self.add_message_to_chat_group(group.clone(), message);
        }
    }
    pub fn add_chat_message_to_spectators(&mut self, message: ChatMessageVariant){
        for spectator in self.spectators.iter_mut(){
            spectator.queued_chat_messages.push(message.clone());
        }
        self.spectator_chat_messages.push(message);
    }

    pub fn send_packet_to_all(&mut self, packet: ToClientPacket){
        for player_ref in PlayerReference::all_players(self){
            player_ref.send_packet(self, packet.clone());
        }
        for spectator in self.spectators.iter(){
            spectator.send_packet(packet.clone());
        }
    }
}

pub mod test {
    use rand::{thread_rng, seq::SliceRandom};

    use super::{Game, settings::Settings, role_list::RoleOutline, player::{PlayerReference, test::mock_player}, phase::PhaseStateMachine, team::Teams, RejectStartReason};

    pub fn mock_game(settings: Settings, number_of_players: usize) -> Result<Game, RejectStartReason> {

        //check settings are not completly off the rails
        if settings.phase_times.game_ends_instantly() {
            return Err(RejectStartReason::ZeroTimeGame);
        }
        
        let mut roles = match settings.role_list.create_random_roles(&settings.excluded_roles){
            Some(roles) => {
                roles
            },
            None => {
                return Err(RejectStartReason::RoleListCannotCreateRoles);
            }
        };
        roles.shuffle(&mut thread_rng());

        let mut players = Vec::new();
        for player_index in 0..number_of_players {
            let new_player = mock_player(
                format!("{}",player_index),
                match roles.get(player_index){
                    Some(role) => *role,
                    None => RoleOutline::Any.get_random_role(&settings.excluded_roles, &roles).expect("Any should have open roles"),
                }
            );
            players.push(new_player);
        }
        drop(roles); // Ensure we don't use the order of roles anywhere

        let mut game = Game{
            ticking: true,
            spectators: Vec::new(),
            spectator_chat_messages: Vec::new(),
            players: players.into_boxed_slice(),
            graves: Vec::new(),
            teams: Teams::default(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings,
        };

        for player_ref in PlayerReference::all_players(&game){
            player_ref.send_join_game_data(&mut game);
        }

        //on role creation needs to be called after all players roles are known
        for player_ref in PlayerReference::all_players(&game){
            let role_data_copy = player_ref.role_state(&game).clone();
            player_ref.set_role(&mut game, role_data_copy);
        }

        Teams::on_team_creation(&mut game);

        Ok(game)
    }
}