use crate::game::{
    player::PlayerReference, 
    role::RoleState, 
    Game
};

#[derive(Clone)]
#[must_use = "Event must be invoked"]
pub struct BeforeRoleSwitch{
    player: PlayerReference,
    old: RoleState,
    new: RoleState,
}
impl BeforeRoleSwitch{
    pub fn new(player: PlayerReference, old: RoleState, new: RoleState) -> Self{
        Self{ player, old, new }
    }
    pub fn invoke(self, game: &mut Game){
        for player in PlayerReference::all_players(game){
            player.before_role_switch(game, self.player, self.old.clone(), self.new.clone());
        }
    }
    pub fn player(&self) -> PlayerReference{
        self.player
    }
    pub fn old_role(&self) -> &RoleState{
        &self.old
    }
    pub fn new_role(&self) -> &RoleState{
        &self.new
    }
}