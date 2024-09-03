use crate::game::{
    components::{cult::Cult, mafia::Mafia},
    player::PlayerReference, 
    role::RoleState, 
    Game
};

#[must_use = "Event must be invoked"]
pub struct OnRoleSwitch{
    player: PlayerReference,
    old: RoleState,
    new: RoleState,
}
impl OnRoleSwitch{
    pub fn new(player: PlayerReference, old: RoleState, new: RoleState) -> Self{
        Self{ player, old, new }
    }
    pub fn invoke(self, game: &mut Game){

        game.on_role_switch(self.player, self.old.role(), self.new.role());

        Cult::on_role_switch(game, self.old.role(), self.new.role());
        Mafia::on_role_switch(game, self.old, self.new);
    }
}