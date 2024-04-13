use crate::packet::ToClientPacket;

use super::{
    chat::{ChatGroup, ChatMessageVariant}, grave::Grave, phase::PhaseType, player::PlayerReference, spectator::spectator_pointer::SpectatorPointer, Game, GameOverReason
};

//Event listerner functions for game defined here
impl Game{
    pub fn on_game_starting(&mut self){
        self.send_packet_to_all(ToClientPacket::StartGame);
        
        //on role creation needs to be called after all players roles are known
        for player_ref in PlayerReference::all_players(self){
            let role_data_copy = player_ref.role_state(self).clone();
            player_ref.set_role(self, role_data_copy);
        }

        for player_ref in PlayerReference::all_players(&self){
            player_ref.send_join_game_data(self);
        }
        for spectator in SpectatorPointer::all_spectators(self){
            spectator.send_join_game_data(self);
        }
    }
    pub fn on_phase_start(&mut self, _phase: PhaseType){
        self.send_packet_to_all(ToClientPacket::Phase { 
            phase: self.current_phase().clone(),
            day_number: self.phase_machine.day_number,
        });
        self.send_packet_to_all(ToClientPacket::PhaseTimeLeft{ seconds_left: self.phase_machine.time_remaining.as_secs() });
        for player in PlayerReference::all_players(self){
            player.send_packet(self, ToClientPacket::YourSendChatGroups { send_chat_groups: 
                player.get_current_send_chat_groups(self)
            });
        }
    }
    pub fn on_any_death(&mut self, _dead_player: PlayerReference){
        for player in PlayerReference::all_players(self){
            player.send_packet(self, ToClientPacket::YourSendChatGroups { send_chat_groups: 
                player.get_current_send_chat_groups(self)
            });
        }
    }
    pub fn on_game_ending(&mut self){
        if self.game_is_over() {
            self.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::GameOver);
            self.send_packet_to_all(ToClientPacket::GameOver{ reason: GameOverReason::Draw });

            for player_ref in PlayerReference::all_players(self){
                self.add_message_to_chat_group(ChatGroup::All,
                    ChatMessageVariant::PlayerWonOrLost{ 
                        player: player_ref.index(), 
                        won: player_ref.get_won_game(self), 
                        role: player_ref.role_state(self).role() 
                    }
                );
            }

            
            self.ticking = false;
        }
    }
    pub fn on_fast_forward(&mut self){
        self.phase_machine.time_remaining = std::time::Duration::from_secs(0);
        
        self.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::PhaseFastForwarded);
        self.send_packet_to_all(ToClientPacket::PhaseTimeLeft{ seconds_left: self.phase_machine.time_remaining.as_secs() });
    }
    pub fn on_grave_added(&mut self, grave: Grave){        
        self.send_packet_to_all(ToClientPacket::AddGrave{grave: grave.clone()});
        self.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::PlayerDied { grave: grave.clone() });

        
        for other_player_ref in PlayerReference::all_players(self){
            other_player_ref.remove_role_label(self, grave.player);
        }
    }
    pub fn on_role_switch(&mut self, actor: PlayerReference){
        for player_ref in PlayerReference::all_players(self){
            player_ref.remove_role_label(self, actor);
        }
    }
}

