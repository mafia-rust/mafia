use crate::game::{components::{cult::Cult, mafia::Mafia}, Game};

pub struct OnGameStart;
impl OnGameStart{
    pub fn invoke(game: &mut Game){
        Mafia::on_game_start(game);
        Cult::on_game_start(game);
    }
}