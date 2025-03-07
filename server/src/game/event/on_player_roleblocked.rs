use crate::game::{ 
    chat::ChatMessageVariant, components::{mafia::Mafia, syndicate_gun_item::SyndicateGunItem}, player::PlayerReference, Game
};

#[must_use = "Event must be invoked"]
pub struct OnPlayerRoleblocked{
    player: PlayerReference,
    invisible: bool,
}
impl OnPlayerRoleblocked{
    pub fn new(player: PlayerReference, invisible: bool) -> Self{
        Self{player, invisible}
    }
    pub fn invoke(self, game: &mut Game){
        self.player.set_night_blocked(game, true);
        if !self.invisible {
            self.player.push_night_message(game,
                ChatMessageVariant::RoleBlocked
            );
        }
        
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_player_roleblocked(game, self.player, self.invisible);
        }
        Mafia::on_player_roleblocked(game, self.player);
        SyndicateGunItem::on_player_roleblocked(game, self.player);
    }
}