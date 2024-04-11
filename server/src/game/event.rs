use crate::packet::ToClientPacket;

use super::{
    chat::{ChatGroup, ChatMessageVariant},
    grave::Grave,
    phase::PhaseType,
    player::PlayerReference,
    spectator::spectator_pointer::SpectatorPointer,
    team::Teams,
    Game,
    GameOverReason
};


// In the future we could use this
//
// pub enum Event{
//     OnPhaseStart(EventOnPhaseStart),
//     OnAnyDeath(EventOnAnyDeath),
//     OnGameEnding(EventOnGameEnding),
// }
// impl Event{
//     pub fn invoke(self, game: &mut Game){
//         match self{
//             Event::OnPhaseStart(event) => event.invoke(game),
//             Event::OnAnyDeath(event) => event.invoke(game),
//             Event::OnGameEnding(event) => event.invoke(game),
//         }
//     }
// }



pub struct OnGameStart;
impl OnGameStart{
    pub fn invoke(game: &mut Game){
        game.on_game_starting();
    }
}

#[must_use = "Event must be invoked"]
pub struct OnPhaseStart{
    pub phase: PhaseType
}
impl OnPhaseStart{
    pub fn new(phase: PhaseType) -> Self{
        Self{ phase }
    }
    pub fn invoke(self, game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_phase_start(game, self.phase);
        }

        Teams::on_phase_start(game, self.phase);

        game.on_phase_start(self.phase);
    }
    pub fn create_and_invoke(game: &mut Game, phase: PhaseType){
        Self::new(phase).invoke(game);
    }
}

#[must_use = "Event must be invoked"]
pub struct OnAnyDeath{
    dead_player: PlayerReference
}
impl OnAnyDeath{
    pub fn new(dead_player: PlayerReference) -> Self{
        Self{ dead_player }
    }
    pub fn invoke(self, game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_any_death(game, self.dead_player)
        }

        Teams::on_any_death(game);

        game.on_any_death(self.dead_player);
    }
}

#[must_use = "Event must be invoked"]
pub struct OnGameEnding;
impl OnGameEnding{
    pub fn invoke(game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_game_ending(game);
        }

        game.on_game_ending();
    }
}

#[must_use = "Event must be invoked"]
pub struct OnFastForward;
impl OnFastForward{
    pub fn invoke(game: &mut Game){
        game.on_fast_forward();
    }
}

pub struct OnGraveAdded{
    pub grave: Grave
}
impl OnGraveAdded{
    pub fn new(grave: Grave) -> Self{
        Self{ grave }
    }
    pub fn invoke(self, game: &mut Game){
        game.on_grave_added(self.grave);
    }
    pub fn create_and_invoke(game: &mut Game, grave: Grave){
        Self::new(grave).invoke(game);
    }
}

//Event listerner functions for game defined here
impl Game{
    fn on_game_starting(&mut self){
        self.send_packet_to_all(ToClientPacket::StartGame);
        
        //on role creation needs to be called after all players roles are known
        for player_ref in PlayerReference::all_players(self){
            let role_data_copy = player_ref.role_state(self).clone();
            player_ref.set_role(self, role_data_copy);
        }

        Teams::on_team_creation(self);

        for player_ref in PlayerReference::all_players(&self){
            player_ref.send_join_game_data(self);
        }
        for spectator in SpectatorPointer::all_spectators(self){
            spectator.send_join_game_data(self);
        }
    }
    fn on_phase_start(&mut self, _phase: PhaseType){
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
    fn on_any_death(&mut self, _dead_player: PlayerReference){
        for player in PlayerReference::all_players(self){
            player.send_packet(self, ToClientPacket::YourSendChatGroups { send_chat_groups: 
                player.get_current_send_chat_groups(self)
            });
        }
    }
    fn on_game_ending(&mut self){
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
    fn on_fast_forward(&mut self){
        self.phase_machine.time_remaining = std::time::Duration::from_secs(0);
        
        self.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::PhaseFastForwarded);
        self.send_packet_to_all(ToClientPacket::PhaseTimeLeft{ seconds_left: self.phase_machine.time_remaining.as_secs() });
    }
    fn on_grave_added(&mut self, grave: Grave){        
        self.send_packet_to_all(ToClientPacket::AddGrave{grave: grave.clone()});
        self.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::PlayerDied { grave: grave.clone() });

        if grave.role.get_role().is_some(){
            for other_player_ref in PlayerReference::all_players(self){
                other_player_ref.insert_role_label(self, grave.player);
            }
        }
    }
}

