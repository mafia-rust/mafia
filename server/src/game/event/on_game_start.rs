use crate::game::{components::{cult::Cult, mafia::Mafia, puppeteer_marionette::PuppeteerMarionette}, modifiers::Modifiers, Game};

#[must_use = "Event must be invoked"]
pub struct OnGameStart;
impl OnGameStart{
    pub fn invoke(game: &mut Game){
        Modifiers::on_game_start(game);
        Mafia::on_game_start(game);
        Cult::on_game_start(game);
        PuppeteerMarionette::on_game_start(game);
    }
}