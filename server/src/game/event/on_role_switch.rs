use crate::game::{components::{ascension::Ascension, cult::Cult, mafia::Mafia}, player::PlayerReference, role::Role, Game};

#[must_use = "Event must be invoked"]
pub struct OnRoleSwitch{
    player: PlayerReference,
    old: Role,
    new: Role,
}
impl OnRoleSwitch{
    pub fn new(player: PlayerReference, old: Role, new: Role) -> Self{
        Self{ player, old, new }
    }
    pub fn invoke(self, game: &mut Game){

        game.on_role_switch(self.player, self.old, self.new);

        Cult::on_role_switch(game, self.old, self.new);
        Mafia::on_role_switch(game, self.old, self.new);
        Ascension::on_role_switch(game);
    }
}