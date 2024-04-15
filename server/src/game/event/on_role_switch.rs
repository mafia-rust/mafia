use crate::game::{player::PlayerReference, role::Role, Game};

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

        game.cult().clone().on_role_switch(game, self.old, self.new);
        game.mafia().clone().on_role_switch(game, self.old, self.new);
    }
}