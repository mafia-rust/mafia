pub mod grave;
pub mod phase;
pub mod player;
pub mod chat;
pub mod role;
pub mod visit;
pub mod verdict;
pub mod role_list;
pub mod settings;
pub mod game_conclusion;
pub mod components;
pub mod available_buttons;
pub mod on_client_message;
pub mod tag;
pub mod event;
pub mod spectator;
pub mod game_listeners;
pub mod attack_power;
pub mod modifiers;
pub mod win_condition;
pub mod role_outline_reference;
pub mod ability_input;

use std::collections::HashMap;
use std::time::Duration;
use components::confused::Confused;
use components::drunk_aura::DrunkAura;
use components::generic_ability::GenericAbilitySaveComponent;
use components::love_linked::LoveLinked;
use components::mafia::Mafia;
use components::pitchfork::Pitchfork;
use components::mafia_recruits::MafiaRecruits;
use components::poison::Poison;
use components::detained::Detained;
use components::insider_group::InsiderGroupID;
use components::insider_group::InsiderGroups;
use components::verdicts_today::VerdictsToday;
use event::on_tick::OnTick;
use modifiers::Modifiers;
use event::before_initial_role_creation::BeforeInitialRoleCreation;
use rand::seq::SliceRandom;
use rand::thread_rng;
use role_outline_reference::OriginallyGeneratedRoleAndPlayer;
use serde::Serialize;

use crate::client_connection::ClientConnection;
use crate::game::event::on_game_start::OnGameStart;
use crate::game::player::PlayerIndex;
use crate::packet::ToClientPacket;
use chat::{ChatMessageVariant, ChatGroup, ChatMessage};
use player::PlayerReference;
use player::Player;
use phase::PhaseStateMachine;
use settings::Settings;
use grave::Grave;
use self::components::{
    arsonist_doused::ArsonistDoused,
    cult::Cult,
    puppeteer_marionette::PuppeteerMarionette
};
use self::game_conclusion::GameConclusion;
use self::event::on_game_ending::OnGameEnding;
use self::event::on_grave_added::OnGraveAdded;
use self::grave::GraveReference;
use self::phase::PhaseState;
use self::player::PlayerInitializeParameters;
use self::spectator::{
    spectator_pointer::{
        SpectatorIndex, SpectatorPointer
    },
    Spectator,
    SpectatorInitializeParameters
};
use self::role::{Role, RoleState};
use self::verdict::Verdict;


pub struct Game {
    pub settings : Settings,

    pub spectators: Vec<Spectator>,
    pub spectator_chat_messages: Vec<ChatMessageVariant>,

    pub roles_originally_generated: Vec<OriginallyGeneratedRoleAndPlayer>,

    pub players: Box<[Player]>,
    pub graves: Vec<Grave>,

    phase_machine : PhaseStateMachine,

    /// Whether the game is still updating phase times
    pub ticking: bool,


    //components with data
    pub generic_ability: GenericAbilitySaveComponent,
    pub cult: Cult,
    pub mafia: Mafia,
    pub arsonist_doused: ArsonistDoused,
    pub puppeteer_marionette: PuppeteerMarionette,
    pub mafia_recruits: MafiaRecruits,
    pub love_linked: LoveLinked,
    pub verdicts_today: VerdictsToday,
    pub pitchfork: Pitchfork,
    pub poison: Poison,
    pub modifiers: Modifiers,
    pub revealed_groups: InsiderGroups,
    pub detained: Detained,
    pub confused: Confused,
    pub drunk_aura: DrunkAura,
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
            let role_list = settings.role_list.clone();


            let roles_to_players = Self::assign_players_to_roles(match role_list.create_random_roles(&settings.enabled_roles){
                Some(roles) => {roles},
                None => {return Err(RejectStartReason::RoleListCannotCreateRoles);}
            });

            let mut roles_to_players_clone = roles_to_players.clone();
            roles_to_players_clone.sort_by(|(_, i), (_,j)| i.cmp(j));
            let shuffled_roles = roles_to_players_clone.into_iter().map(|(r,_)|r).collect::<Vec<Role>>();            


            let mut new_players = Vec::new();
            for (player_index, player) in players.iter().enumerate() {
                let ClientConnection::Connected(ref sender) = player.connection else {
                    return Err(RejectStartReason::PlayerDisconnected)
                };
                let new_player = Player::new(
                    player.name.clone(),
                    sender.clone(),
                    match shuffled_roles.get(player_index){
                        Some(role) => *role,
                        None => return Err(RejectStartReason::RoleListTooSmall),
                    }
                );
                new_players.push(new_player);
            }
            drop(shuffled_roles); // Ensure we don't use the order of roles anywhere

            let game = Self{
                roles_originally_generated: roles_to_players.into_iter().map(|(r,i)|(r,PlayerReference::new_unchecked(i))).collect(),
                ticking: true,
                spectators: spectators.clone().into_iter().map(Spectator::new).collect(),
                spectator_chat_messages: Vec::new(),
                players: new_players.into_boxed_slice(),
                graves: Vec::new(),
                phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
                modifiers: Modifiers::default_from_settings(settings.enabled_modifiers.clone()),
                settings,

                generic_ability: GenericAbilitySaveComponent::default(),
                cult: Cult::default(),
                mafia: Mafia,
                arsonist_doused: ArsonistDoused::default(),
                puppeteer_marionette: PuppeteerMarionette::default(),
                mafia_recruits: MafiaRecruits::default(),
                love_linked: LoveLinked::default(),
                verdicts_today: VerdictsToday::default(),
                pitchfork: Pitchfork::default(),
                poison: Poison::default(),

                revealed_groups: InsiderGroups::default(),
                detained: Detained::default(),
                confused: Confused::default(),
                drunk_aura: DrunkAura::default(),
            };

            if !game.game_is_over() {
                break game;
            }
            role_generation_tries += 1;
        };

        if game.game_is_over() {
            return Err(RejectStartReason::RoleListCannotCreateRoles);
        }
        
        game.send_packet_to_all(ToClientPacket::StartGame);

        //set wincons and revealed groups
        for player in PlayerReference::all_players(&game){
            let role_data = player.role_state(&game).clone();

            player.set_win_condition(&mut game, role_data.clone().default_win_condition());
        
            InsiderGroupID::start_game_set_player_revealed_groups(
                role_data.clone().default_revealed_groups(),
                &mut game,
                player
            );
        }

        BeforeInitialRoleCreation::invoke(&mut game);
        
        //on role creation needs to be called after all players roles are known
        //trigger role event listeners
        for player_ref in PlayerReference::all_players(&game){
            player_ref.initial_role_creation(&mut game);
        }

        for player_ref in PlayerReference::all_players(&game){
            player_ref.send_join_game_data(&mut game);
        }
        for spectator in SpectatorPointer::all_spectators(&game){
            spectator.send_join_game_data(&mut game);
        }

        //reveal groups
        for group in InsiderGroupID::all() {
            group.reveal_group_players(&mut game);
        }

        //on game start needs to be called after all players have joined
        OnGameStart::invoke(&mut game);

        Ok(game)
    }
    fn assign_players_to_roles(roles: Vec<Role>)->Vec<(Role, PlayerIndex)>{
        let mut player_indices: Vec<PlayerIndex> = (0..roles.len() as PlayerIndex).collect();
        player_indices.shuffle(&mut thread_rng());
        roles.into_iter().zip(player_indices).collect()
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
            if let RoleState::Politician(politician) = player_ref.role_state(self).clone(){
                if politician.revealed {
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

        let &PhaseState::Nomination { trials_left, .. } = self.current_phase() else {return};

        let mut voted_player_votes: HashMap<PlayerReference, u8> = HashMap::new();

        for player in PlayerReference::all_players(self){
            if !player.alive(self) { continue }

            let Some(voted_player) = player.chosen_vote(self) else { continue };

            let mut voting_power = 1;
            if let RoleState::Mayor(mayor) = player.role_state(self).clone() {
                if mayor.revealed {
                    voting_power = 3;
                }
            }
            else if let RoleState::Politician(politician) = player.role_state(self).clone() {
                if politician.revealed {
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
            if self.nomination_votes_is_enough(*votes){
                next_player_on_trial = Some(*player);
                break;
            }
        }
        
        if let Some(player_on_trial) = next_player_on_trial {
            self.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: player_on_trial.index() } );
            
            PhaseStateMachine::next_phase(self, Some(PhaseState::Testimony {
                trials_left: trials_left-1, 
                player_on_trial, 
                nomination_time_remaining: self.phase_machine.get_time_remaining()
            }));
        }
    }
    pub fn nomination_votes_is_enough(&self, votes: u8)->bool{
        votes >= self.nomination_votes_required()
    }
    pub fn nomination_votes_required(&self)->u8{
        1 + (
            PlayerReference::all_players(self)
                .filter(|p| p.alive(self) && !p.forfeit_vote(self))
                .count() / 2
        ) as u8
    }

    pub fn game_is_over(&self) -> bool {
        if let Some(_) = GameConclusion::game_is_over(self){
            true
        }else{
            false
        }
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
            PhaseStateMachine::next_phase(self, None);
        }
        PlayerReference::all_players(self).for_each(|p|p.tick(self, time_passed));
        SpectatorPointer::all_spectators(self).for_each(|s|s.tick(self, time_passed));

        self.phase_machine.time_remaining = self.phase_machine.time_remaining.saturating_sub(time_passed);

        OnTick::new().invoke(self);
    }

    pub fn add_grave(&mut self, grave: Grave){
        self.graves.push(grave.clone());
        if let Some(grave_ref) = GraveReference::new(
            self, 
            self.graves.len()
                .saturating_sub(1)
                .try_into()
                .expect("There can not be more than u8::MAX graves"))
        {
            OnGraveAdded::new(grave_ref).invoke(self);
        }
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

    pub fn add_spectator(&mut self, params: SpectatorInitializeParameters) -> SpectatorIndex {
        self.spectators.push(Spectator::new(params));
        let spectator_pointer = SpectatorPointer::new(self.spectators.len() as SpectatorIndex - 1);

        spectator_pointer.send_join_game_data(self);

        spectator_pointer.index
    }
    pub fn remove_spectator(&mut self, i: SpectatorIndex){
        self.spectators.remove(i as usize);
    }

    pub fn send_packet_to_all(&self, packet: ToClientPacket){
        for player_ref in PlayerReference::all_players(self){
            player_ref.send_packet(self, packet.clone());
        }
        for spectator in self.spectators.iter(){
            spectator.send_packet(packet.clone());
        }
    }
}

pub mod test {

    use super::{
        components::{arsonist_doused::ArsonistDoused, cult::Cult, generic_ability::GenericAbilitySaveComponent, insider_group::InsiderGroupID, love_linked::LoveLinked, mafia::Mafia, mafia_recruits::MafiaRecruits, pitchfork::Pitchfork, poison::Poison, puppeteer_marionette::PuppeteerMarionette, verdicts_today::VerdictsToday},
        event::{before_initial_role_creation::BeforeInitialRoleCreation, on_game_start::OnGameStart},
        phase::PhaseStateMachine,
        player::{test::mock_player, PlayerIndex, PlayerReference},
        role::Role,
        settings::Settings, 
        Game,
        RejectStartReason
    };


    pub fn mock_game(settings: Settings, number_of_players: usize) -> Result<Game, RejectStartReason> {

        //check settings are not completly off the rails
        if settings.phase_times.game_ends_instantly() {
            return Err(RejectStartReason::ZeroTimeGame);
        }

        let settings = settings.clone();
        let role_list = settings.role_list.clone();
        
        let roles_to_players = assign_players_to_roles(match role_list.create_random_roles(&settings.enabled_roles){
            Some(roles) => {roles},
            None => {return Err(RejectStartReason::RoleListCannotCreateRoles);}
        });
        
        let mut roles_to_players_clone = roles_to_players.clone();
        roles_to_players_clone.sort_by(|(_, i), (_,j)| i.cmp(j));
        let shuffled_roles = roles_to_players_clone.into_iter().map(|(r,_)|r).collect::<Vec<Role>>();


        let mut players = Vec::new();
        for player_index in 0..number_of_players {
            let new_player = mock_player(
                format!("{}",player_index),
                match shuffled_roles.get(player_index){
                    Some(role) => *role,
                    None => return Err(RejectStartReason::RoleListTooSmall),
                }
            );
            players.push(new_player);
        }
        drop(shuffled_roles); // Ensure we don't use the order of roles anywhere

        let mut game = Game{
            roles_originally_generated: roles_to_players.into_iter().map(|(r,i)|(r,PlayerReference::new_unchecked(i))).collect(),
            ticking: true,
            spectators: Vec::new(),
            spectator_chat_messages: Vec::new(),
            players: players.into_boxed_slice(),
            graves: Vec::new(),
            phase_machine: PhaseStateMachine::new(settings.phase_times.clone()),
            settings,

            generic_ability: GenericAbilitySaveComponent::default(),
            cult: Cult::default(),
            mafia: Mafia,
            arsonist_doused: ArsonistDoused::default(),
            puppeteer_marionette: PuppeteerMarionette::default(),
            mafia_recruits: MafiaRecruits::default(),
            love_linked: LoveLinked::default(),
            verdicts_today: VerdictsToday::default(),
            pitchfork: Pitchfork::default(),
            poison: Poison::default(),
            modifiers: Default::default(),
            revealed_groups: Default::default(),
            detained: Default::default(),
            confused: Default::default(),
            drunk_aura: Default::default(),
        };

        //set wincons and revealed groups
        for player in PlayerReference::all_players(&game){
            let role_data = player.role_state(&game).clone();

            player.set_win_condition(&mut game, role_data.clone().default_win_condition());
        
            InsiderGroupID::start_game_set_player_revealed_groups(
                role_data.clone().default_revealed_groups(),
                &mut game,
                player
            );
        }
        
        BeforeInitialRoleCreation::invoke(&mut game);

        //on role creation needs to be called after all players roles are known
        for player_ref in PlayerReference::all_players(&game){
            player_ref.initial_role_creation(&mut game);
        }

        OnGameStart::invoke(&mut game);

        Ok(game)
    }
    fn assign_players_to_roles(roles: Vec<Role>)->Vec<(Role, PlayerIndex)>{
        let player_indices: Vec<PlayerIndex> = (0..roles.len() as PlayerIndex).collect();
        roles.into_iter().zip(player_indices).collect()
    }

}