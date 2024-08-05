use crate::packet::ToClientPacket;

use super::{
    chat::ChatMessageVariant, player_group::PlayerGroup, grave::GraveReference, phase::PhaseType, player::PlayerReference, role::Role, Game, GameOverReason
};

//Event listerner functions for game defined here
impl Game{
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
            ChatMessageVariant::GameOver.send_to(self, PlayerGroup::All);
            self.add_message(PlayerGroup::All, ChatMessageVariant::GameOver);
            self.send_packet_to_all(ToClientPacket::GameOver{ reason: GameOverReason::Draw });

            for player_ref in PlayerReference::all_players(self){
                self.add_message(PlayerGroup::All,
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
        
        self.add_message(PlayerGroup::All, ChatMessageVariant::PhaseFastForwarded);
        self.send_packet_to_all(ToClientPacket::PhaseTimeLeft{ seconds_left: self.phase_machine.time_remaining.as_secs() });
    }
    pub fn on_grave_added(&mut self, grave: GraveReference){   
        let grave = grave.deref(self).clone();     
        self.send_packet_to_all(ToClientPacket::AddGrave{grave: grave.clone()});
        self.add_message(PlayerGroup::All, ChatMessageVariant::PlayerDied { grave: grave.clone() });

        grave.player.conceal_role(self, PlayerGroup::All);
    }
    pub fn on_role_switch(&mut self, actor: PlayerReference, old: Role, new: Role){
        if old == new {return;}

        actor.conceal_role(self, PlayerGroup::All);
    }
}

