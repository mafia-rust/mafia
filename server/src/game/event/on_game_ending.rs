use crate::game::{components::puppeteer_marionette::PuppeteerMarionette, player::PlayerReference, Game};

#[must_use = "Event must be invoked"]
pub struct OnGameEnding;
impl OnGameEnding{
    pub fn invoke(game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_game_ending(game);
        }

        PuppeteerMarionette::on_game_ending(game);
        game.on_game_ending();
    }
}
