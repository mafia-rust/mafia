use crate::game::{player::PlayerReference, role::Role, Game};

pub struct OnRoleSwitch{
    player: PlayerReference,
    new_role: Role
}
impl OnRoleSwitch{
    pub fn new(player: PlayerReference, new_role: Role) -> Self{
        Self{ player, new_role }
    }
    pub fn invoke(self, game: &mut Game){

        game.cult().on_member_role_switch(game, self.player);
        game.mafia().on_member_role_switch(game,self.player);
    }
    pub fn create_and_invoke(game: &mut Game, player: PlayerReference, new_role: Role){
        Self::new(player, new_role).invoke(game);
    }
}