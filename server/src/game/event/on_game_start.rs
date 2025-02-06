use crate::game::{components::{cult::Cult, mafia::Mafia, mafia_recruits::MafiaRecruits, puppeteer_marionette::PuppeteerMarionette}, modifiers::Modifiers, player::PlayerReference, role::GetClientRoleState, Game};
use crate::database_resources::database_queries;

#[must_use = "Event must be invoked"]
#[derive(Clone)]
pub struct OnGameStart;
impl OnGameStart {
    pub fn invoke(game: &mut Game) {
        Modifiers::on_game_start(game);

        for player in PlayerReference::all_players(game) {
            player.on_game_start(game);
        }
        Mafia::on_game_start(game);
        Cult::on_game_start(game);
        PuppeteerMarionette::on_game_start(game);
        MafiaRecruits::on_game_start(game);
        tokio::spawn(async move {
            database_queries::on_game_start().await.unwrap();
        });
    }
}