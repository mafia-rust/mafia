use crate::game::{
    components::{cult::Cult, mafia::Mafia}, 
    player::PlayerReference, 
    Game
};

#[must_use = "Event must be invoked"]
pub struct OnAnyDeath{
    dead_player: PlayerReference,
}
impl OnAnyDeath{
    pub fn new(dead_player: PlayerReference) -> Self{
        Self{dead_player}
    }
    pub fn invoke(self, game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_any_death(game, self.dead_player)
        }

        Mafia::on_any_death(game, self.dead_player);
        Cult::on_any_death(game);

        game.on_any_death(self.dead_player);
    }
}