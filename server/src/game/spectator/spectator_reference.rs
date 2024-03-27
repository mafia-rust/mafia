use crate::{
    game::{chat::{ChatGroup, ChatMessage}, phase::PhaseState, player::PlayerReference, Game, GameOverReason}, 
    packet::ToClientPacket
};

use super::Spectator;

///
/// This does not garuntee that the spectator exists
pub struct SpectatorPointer {
    pub index: u8,
}
impl SpectatorPointer {
    //THESE FUNCTIONS SHOULD NOT TAKE &self
    //Instead there should be some sort of spectator reference like player reference
    //This is because self and game will make a double borrow

    pub fn deref_mut<'a>(&self, game: &'a mut Game)->Option<&'a mut Spectator>{
        game.spectators.get_mut(self.index as usize)
    }
    pub fn deref<'a>(&self, game: &'a Game)->Option<&'a Spectator>{
        game.spectators.get(self.index as usize)
    }

    pub fn send_packet(&self, game: &Game, packet: ToClientPacket){
        match self.deref(game) {
            Some(s) => s.send_packet(packet),
            None => (),
        }
    }
    pub fn send_packets(&self, game: &Game, packets: Vec<ToClientPacket>){
        match self.deref(game){
            Some(s) => s.send_packets(packets),
            None => (),
        }
    }
    pub fn send_repeating_data(&mut self, game: &mut Game){
        self.send_chat_messages(game);
    }
    pub fn send_join_game_data(&self, game: &mut Game){
        // General
        self.send_packets(game, vec![
            ToClientPacket::GamePlayers{ 
                players: PlayerReference::all_players(game).map(|p|p.name(game).clone()).collect()
            },
            ToClientPacket::ExcludedRoles { roles: game.settings.excluded_roles.clone() },
            ToClientPacket::RoleList {role_list: game.settings.role_list.clone()},
            ToClientPacket::PlayerAlive{
                alive: PlayerReference::all_players(game).map(|p|p.alive(game)).collect()
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

        // // Player specific
        // self.requeue_chat_messages(game);

        self.send_packets(game, vec![
            ToClientPacket::Phase { 
                phase: game.current_phase().phase(),
                day_number: game.phase_machine.day_number 
            },
            ToClientPacket::PhaseTimeLeft { seconds_left: game.phase_machine.time_remaining.as_secs() }
        ]);
    }



    pub fn send_chat_messages(&mut self, game: &mut Game){
        
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
                .map(|p|ChatMessage::new_non_private(p, ChatGroup::All))
                .collect() 
            }
        );
        

        self.send_chat_messages(game);
    }
}