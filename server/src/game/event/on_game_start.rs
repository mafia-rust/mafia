use crate::game::Game;

pub struct OnGameStart;
impl OnGameStart{
    pub fn invoke(game: &mut Game){
        game.mafia().clone().on_game_start(game);
        game.cult().clone().on_game_start(game);
    }
}