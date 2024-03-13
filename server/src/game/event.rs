use crate::packet::ToClientPacket;

use super::{
    chat::{ChatGroup, ChatMessageVariant},
    phase::PhaseType,
    player::PlayerReference,
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
    pub fn create_and_invoke(phase: PhaseType, game: &mut Game){
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

//Event listerner functions for game defined here
impl Game{
    fn on_phase_start(&mut self, _phase: PhaseType){
        self.send_packet_to_all(ToClientPacket::Phase { 
            phase: self.current_phase().phase(),
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
                    });
            }

            
            self.ticking = false;
        }
    }
}

