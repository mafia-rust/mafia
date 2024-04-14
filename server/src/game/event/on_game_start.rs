use crate::game::Game;

pub struct OnGameStart;
impl OnGameStart{
    pub fn invoke(game: &mut Game){
        game.on_game_starting();
        
        game.mafia().clone().on_game_start(game);
        game.cult().clone().on_game_start(game);
    }
}