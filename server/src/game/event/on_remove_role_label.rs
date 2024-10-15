use crate::game::{
    components::revealed_group::RevealedGroups, player::PlayerReference, Game
};

#[must_use = "Event must be invoked"]
pub struct OnRemoveRoleLabel{
    player: PlayerReference,
    concealed_player: PlayerReference
}
impl OnRemoveRoleLabel{
    pub fn new(player: PlayerReference, concealed_player: PlayerReference) -> Self{
        Self{ player, concealed_player }
    }
    pub fn invoke(self, game: &mut Game){
        RevealedGroups::on_remove_role_label(game, self.player, self.concealed_player);
    }
}