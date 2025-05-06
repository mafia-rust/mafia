use crate::game::{
    components::insider_group::InsiderGroups, player::PlayerReference, Game
};

#[must_use = "Event must be invoked"]
pub struct OnConcealRole{
    player: PlayerReference,
    concealed_player: PlayerReference
}
impl OnConcealRole{
    pub fn new(player: PlayerReference, concealed_player: PlayerReference) -> Self{
        Self{ player, concealed_player }
    }
    pub fn invoke(self, game: &mut Game){
        PlayerReference::all_players(game).for_each(|p|p.on_conceal_role(game, self.player, self.concealed_player));
        InsiderGroups::on_conceal_role(game, self.player, self.concealed_player);
    }
}