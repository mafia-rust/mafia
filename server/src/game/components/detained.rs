use std::collections::HashSet;

use crate::game::{chat::ChatMessageVariant, event::on_midnight::{MidnightVariables, OnMidnight, OnMidnightPriority}, phase::PhaseType, player::PlayerReference, Game};

use super::insider_group::InsiderGroupID;

#[derive(Default)]
pub struct Detained{
    //resets every obituary
    players: HashSet<PlayerReference>,
}
impl Detained{
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        if phase == PhaseType::Obituary {
            Detained::clear_detain(game);
        }
    }
    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, midnight_variables: &mut MidnightVariables, priority: OnMidnightPriority){
        match priority {
            OnMidnightPriority::Ward => {
                for player in PlayerReference::all_players(game){
                    if Self::is_detained(game, player){
                        player.ward(game, midnight_variables, &[]);
                    }
                }
            }
            OnMidnightPriority::Roleblock => {
                for player in PlayerReference::all_players(game){
                    if Self::is_detained(game, player){
                        player.roleblock(game, midnight_variables, true);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn add_detain(game: &mut Game, player: PlayerReference){
        InsiderGroupID::send_message_in_available_insider_chat_or_private(
            game,
            player,
            ChatMessageVariant::JailedSomeone { player_index: player.index() },
            true
        );
        game.detained.players.insert(player);
    }
    pub fn remove_detain(game: &mut Game, player: PlayerReference){
        game.detained.players.remove(&player);
    }
    pub fn clear_detain(game: &mut Game){
        game.detained.players.clear();
    }
    
    pub fn is_detained(game: &Game, player: PlayerReference)->bool{
        game.detained.players.contains(&player)
    }
}