use std::collections::HashSet;

use crate::game::{chat::{ChatGroup, ChatMessageVariant}, phase::PhaseType, player::PlayerReference, role::Priority, Game};

#[derive(Default)]
pub struct Detained{
    //resets every obituary
    pub players: HashSet<PlayerReference>,
}
impl Detained{
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        match phase {
            PhaseType::Obituary => {
                Detained::clear_detain(game);
            }
            _ => {}
        }
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        match priority {
            Priority::Ward => {
                for player in PlayerReference::all_players(game){
                    if Self::is_detained(game, player){
                        player.ward(game);
                    }
                }
            }
            Priority::Roleblock => {
                for player in PlayerReference::all_players(game){
                    if Self::is_detained(game, player){
                        // player.roleblock(game, true);
                    }
                }
            }
            _ => {}
        }
    }
    pub fn detained<'a>(game: &'a Game)->&'a Detained{
        &game.detained
    }
    pub fn detained_mut<'a>(game: &'a mut Game)->&'a mut Detained{
        &mut game.detained
    }

    pub fn add_detain(game: &mut Game, player: PlayerReference){
        let mut message_sent = false;
        for chat_group in player.get_current_send_chat_groups(game){
            match chat_group {
                ChatGroup::All | ChatGroup::Jail | ChatGroup::Kidnapped | ChatGroup::Interview | ChatGroup::Dead => {},
                ChatGroup::Mafia | ChatGroup::Cult | ChatGroup::Puppeteer => {
                    game.add_message_to_chat_group(
                        chat_group,
                        ChatMessageVariant::JailedSomeone { player_index: player.index() }
                    );
                    message_sent = true;
                },
            }
        }
        if !message_sent {
            player.add_private_chat_message(game,
                ChatMessageVariant::JailedSomeone { player_index: player.index() }
            );
        }
    
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