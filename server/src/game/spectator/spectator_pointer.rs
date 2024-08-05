use std::time::Duration;

use crate::{
    client_connection::ClientConnection, game::{chat::ChatMessage, phase::PhaseState, player::PlayerReference, player_group::PlayerGroup, Game, GameOverReason}, packet::ToClientPacket
};

use super::Spectator;

pub type SpectatorIndex = u8;
///
/// This does not guarantee that the spectator exists
pub struct SpectatorPointer {
    pub index: SpectatorIndex,
}
impl SpectatorPointer {
    pub fn new(index: SpectatorIndex) -> Self {
        SpectatorPointer { index }
    }

    pub fn deref_mut<'a>(&self, game: &'a mut Game)->Option<&'a mut Spectator>{
        game.spectators.get_mut(self.index as usize)
    }
    pub fn deref<'a>(&self, game: &'a Game)->Option<&'a Spectator>{
        game.spectators.get(self.index as usize)
    }

    pub fn host(&self, game: &Game)->bool {
        self.deref(game).map(|s|s.host).unwrap_or(false)
    }
    pub fn connection(&self, game: &Game) -> ClientConnection {
        self.deref(game).map(|s|s.connection.clone()).unwrap_or(ClientConnection::Disconnected)
    }

    pub fn send_packet(&self, game: &Game, packet: ToClientPacket){
        if let Some(s) = self.deref(game) { 
            s.send_packet(packet)
        }
    }
    pub fn send_packets(&self, game: &Game, packets: Vec<ToClientPacket>){
        if let Some(s) = self.deref(game) { 
            s.send_packets(packets) 
        }
    }

    pub fn all_spectators(game: &Game) -> SpectatorPointerIterator {
        SpectatorPointerIterator {
            current: 0,
            end: game.spectators.len() as SpectatorIndex
        }
    }


    pub fn tick(&self, game: &mut Game, _time_passed: Duration){

        let s = match self.deref_mut(game){
            Some(s) => s,
            None => return
        };

        if let ClientConnection::Connected(_) = s.connection {
            self.send_repeating_data(game)
        }
    }
    pub fn send_repeating_data(&self, game: &mut Game){
        self.send_chat_messages(game);
    }
    pub fn send_join_game_data(&self, game: &mut Game){
        // General
        self.send_packets(game, vec![
            ToClientPacket::GamePlayers{ 
                players: PlayerReference::all_players(game).map(|p|p.name(game).clone()).collect()
            },
            ToClientPacket::ExcludedRoles { roles: game.settings.excluded_roles.clone().into_iter().collect() },
            ToClientPacket::RoleList {role_list: game.settings.role_list.clone()},
            ToClientPacket::PlayerAlive{
                alive: PlayerReference::all_players(game).map(|p|p.alive(game)).collect()
            },
            ToClientPacket::PhaseTimes {
                phase_time_settings: game.settings.phase_times.clone()
            }
        ]);

        if !game.ticking {
            self.send_packet(game, ToClientPacket::GameOver { reason: GameOverReason::Draw })
        }

        if let PhaseState::Testimony { player_on_trial, .. }
            | PhaseState::Judgement { player_on_trial, .. }
            | PhaseState::FinalWords { player_on_trial } = game.current_phase() {
            self.send_packet(game, ToClientPacket::PlayerOnTrial{
                player_index: player_on_trial.index()
            });
        }
        let votes_packet = ToClientPacket::new_player_votes(game);
        self.send_packet(game, votes_packet);
        for grave in game.graves.iter(){
            self.send_packet(game, ToClientPacket::AddGrave { grave: grave.clone() });
        }

        self.send_packets(game, vec![
            ToClientPacket::Phase { 
                phase: game.current_phase().clone(),
                day_number: game.phase_machine.day_number 
            },
            ToClientPacket::PhaseTimeLeft { seconds_left: game.phase_machine.time_remaining.as_secs() }
        ]);

        self.requeue_chat_messages(game);
        self.send_chat_messages(game);
    }

    pub fn requeue_chat_messages(&self, game: &mut Game){
        let msgs = game.spectator_chat_messages.clone();

        let s = match self.deref_mut(game){
            Some(s)=>s,
            None=> return
        };

        for msg in msgs.into_iter(){
            s.queued_chat_messages.push(msg);
        }
    }

    pub fn send_chat_messages(&self, game: &mut Game){
        
        let s = match self.deref_mut(game){
            Some(s)=>s,
            None=> return
        };

        if s.queued_chat_messages.is_empty() {
            return;
        }
        
        let mut chat_messages_out = vec![];

        // Send in chunks
        for _ in 0..5 {
            let msg_option = s.queued_chat_messages.first();
            if let Some(msg) = msg_option{
                chat_messages_out.push(msg.clone());
                s.queued_chat_messages.remove(0);
            }else{ break; }
        }
        
        self.send_packet(game, ToClientPacket::AddChatMessages { chat_messages: chat_messages_out
                .into_iter()
                .map(|p|ChatMessage::new(p, PlayerGroup::All))
                .collect() 
            }
        );
        

        self.send_chat_messages(game);
    }
}

pub struct SpectatorPointerIterator {
    current: u8,
    end: u8
}

impl Iterator for SpectatorPointerIterator {
    type Item = SpectatorPointer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            None
        } else {
            let ret = SpectatorPointer::new(self.current);
            self.current += 1;
            Some(ret)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.end - self.current) as usize;
        (size, Some(size))
    }
}

impl ExactSizeIterator for SpectatorPointerIterator {}