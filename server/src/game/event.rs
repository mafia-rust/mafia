use crate::packet::ToClientPacket;

use super::{phase::PhaseType, player::PlayerReference, team::Teams, Game};




impl Game{
    pub fn invoke_on_phase_start(&mut self, phase: PhaseType){
        for player in PlayerReference::all_players(self){
            player.on_phase_start(self, phase);
        }
        Teams::on_phase_start(self, self.current_phase().phase());
    }
    pub fn invoke_on_any_death(&mut self, dead_player: PlayerReference){
        for player_ref in PlayerReference::all_players(self){
            player_ref.on_any_death(self, dead_player)
        }
        Teams::on_any_death(self);



        
        for player in PlayerReference::all_players(self){
            player.send_packet(self, ToClientPacket::YourSendChatGroups { send_chat_groups: 
                player.get_current_send_chat_groups(self)
            });
        }
    }
    pub fn invoke_on_game_ending(&mut self){
        for player in PlayerReference::all_players(self){
            player.on_game_ending(self);
        }
    }
}

