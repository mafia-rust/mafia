use crate::game::{components::{cult::Cult, mafia::Mafia, mafia_recruits::MafiaRecruits, puppeteer_marionette::PuppeteerMarionette}, player::PlayerReference, Game};

#[must_use = "Event must be invoked"]
pub struct OnGameStart;
impl OnGameStart{
    pub fn invoke(game: &mut Game){
        for player in PlayerReference::all_players(game){
            player.on_game_start(game);
        }
        Mafia::on_game_start(game);
        Cult::on_game_start(game);
        PuppeteerMarionette::on_game_start(game);
        MafiaRecruits::on_game_start(game);
    }
}