use crate::game::{player::PlayerReference, Game};

pub struct OnRoleSwitch{
    player: PlayerReference,
}
impl OnRoleSwitch{
    pub fn new(player: PlayerReference) -> Self{
        Self{ player }
    }
    pub fn invoke(self, game: &mut Game){

        game.cult().clone().on_role_switch(game, self.player);
        game.mafia().clone().on_role_switch(game, self.player);
    }
    pub fn create_and_invoke(game: &mut Game, player: PlayerReference){
        Self::new(player).invoke(game);
    }
}