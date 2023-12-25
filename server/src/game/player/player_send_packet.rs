use std::time::Duration;

use crate::{game::{Game, available_buttons::AvailableButtons, phase::PhaseState, GameOverReason, grave::{Grave, GraveDeathCause}}, packet::ToClientPacket, websocket_connections::connection::ClientSender, lobby::GAME_DISCONNECT_TIMER_SECS};

use super::{PlayerReference, ClientConnection};

impl PlayerReference{
    pub fn connect(&self, game: &mut Game, sender: ClientSender){
        self.deref_mut(game).connection = ClientConnection::Connected(sender);
        self.send_join_game_data(game);
    }
    pub fn lose_connection(&self, game: &mut Game){
        self.deref_mut(game).connection = ClientConnection::CouldReconnect { disconnect_timer: Duration::from_secs(GAME_DISCONNECT_TIMER_SECS) };
    }
    pub fn leave(&self, game: &mut Game) {
        if self.alive(game){
            let mut grave = Grave::from_player_suicide(game, *self);
            grave.death_cause = GraveDeathCause::DisconnectedFromLife;
            self.die(game, grave);
        }
        self.deref_mut(game).connection = ClientConnection::Disconnected;
    }
    pub fn is_connected(&self, game: &Game) -> bool {
        matches!(self.deref(game).connection, ClientConnection::Connected(_))
    }
    pub fn has_lost_connection(&self, game: &Game) -> bool {
        matches!(self.deref(game).connection, ClientConnection::CouldReconnect {..})
    }
    pub fn has_left(&self, game: &Game) -> bool {
        matches!(self.deref(game).connection, ClientConnection::Disconnected)
    }

    pub fn send_packet(&self, game: &Game, packet: ToClientPacket){
        if let ClientConnection::Connected(sender) = &self.deref(game).connection{
            sender.send(packet);
        }
    }
    pub fn send_packets(&self, game: &Game, packets: Vec<ToClientPacket>){
        for packet in packets{
            self.send_packet(game, packet);
        }
    }
    pub fn send_repeating_data(&self, game: &mut Game){
        self.send_chat_messages(game);
        self.send_available_buttons(game);
    }
    pub fn send_join_game_data(&self, game: &mut Game){
        // General
        self.send_packets(game, vec![
            ToClientPacket::GamePlayers{ 
                players: PlayerReference::all_players(game).into_iter().map(|p|p.name(game).clone()).collect()
            },
            ToClientPacket::ExcludedRoles { roles: game.settings.excluded_roles.clone() },
            ToClientPacket::RoleList {role_list: game.settings.role_list.clone()},
            ToClientPacket::Phase { 
                phase: game.current_phase().phase(),
                seconds_left: game.phase_machine.time_remaining.as_secs(), 
                day_number: game.phase_machine.day_number 
            },
            ToClientPacket::PlayerAlive{
                alive: PlayerReference::all_players(game).into_iter().map(|p|p.alive(game)).collect()
            }
        ]);

        if !game.ticking {
            self.send_packet(game, ToClientPacket::GameOver { reason: GameOverReason::Draw })
        }

        if let PhaseState::Testimony { player_on_trial, .. }
            | PhaseState::Judgement { player_on_trial, .. }
            | PhaseState::Evening { player_on_trial: Some(player_on_trial) } = game.current_phase() {
            self.send_packet(game, ToClientPacket::PlayerOnTrial{
                player_index: player_on_trial.index()
            });
        }
        let votes_packet = ToClientPacket::new_player_votes(game);
        self.send_packet(game, votes_packet);
        for grave in game.graves.iter(){
            self.send_packet(game, ToClientPacket::AddGrave { grave: grave.clone() });
        }

        // Player specific
        self.requeue_chat_messages(game);

        self.send_packets(game, vec![
            ToClientPacket::YourPlayerIndex { 
                player_index: self.index() 
            },
            ToClientPacket::YourRoleState {
                role_state: self.role_state(game).clone()
            },
            ToClientPacket::YourRoleLabels { 
                role_labels: PlayerReference::ref_map_to_index(self.role_labels(game).clone()) 
            },
            ToClientPacket::YourPlayerTags { 
                player_tags: PlayerReference::ref_map_to_index(self.player_tags(game).clone())
            },
            ToClientPacket::YourTarget{
                player_indices: PlayerReference::ref_vec_to_index(self.chosen_targets(game))
            },
            ToClientPacket::YourJudgement{
                verdict: self.verdict(game)
            },
            ToClientPacket::YourVoting{ 
                player_index: PlayerReference::ref_option_to_index(&self.chosen_vote(game))
            },
            ToClientPacket::YourWill{
                will: self.will(game).clone()
            },
            ToClientPacket::YourNotes{
                notes: self.notes(game).clone()
            },
            ToClientPacket::YourButtons{
                buttons: AvailableButtons::from_player(game, *self)
            }
        ]);
    }



    pub fn send_chat_messages(&self, game: &mut Game){
        
        if self.deref(game).queued_chat_messages.is_empty() {
            return;
        }
        
        let mut chat_messages_out = vec![];

        // Send in chunks
        for _ in 0..5 {
            let msg_option = self.deref(game).queued_chat_messages.get(0);
            if let Some(msg) = msg_option{
                chat_messages_out.push(msg.clone());
                self.deref_mut(game).queued_chat_messages.remove(0);
            }else{ break; }
        }
        
        self.send_packet(game, ToClientPacket::AddChatMessages { chat_messages: chat_messages_out });
        

        self.send_chat_messages(game);
    }
    #[allow(unused)]
    fn requeue_chat_messages(&self, game: &mut Game){
        for msg in self.deref(game).chat_messages.clone().into_iter(){
            self.deref_mut(game).queued_chat_messages.push(msg);
        };
    }   

    fn send_available_buttons(&self, game: &mut Game){
        let new_buttons = AvailableButtons::from_player(game, *self);
        if new_buttons == self.deref(game).last_sent_buttons{
            return;
        }
        
        self.send_packet(game, ToClientPacket::YourButtons { buttons: new_buttons.clone() });
        self.deref_mut(game).last_sent_buttons = new_buttons
    }

}

