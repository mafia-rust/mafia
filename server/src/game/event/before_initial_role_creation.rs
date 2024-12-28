use crate::game::{modifiers::Modifiers, player::PlayerReference, Game};

#[must_use = "Event must be invoked"]
pub struct BeforeInitialRoleCreation;
impl BeforeInitialRoleCreation{
    pub fn invoke(game: &mut Game){
        Modifiers::before_initial_role_creation(game);

        for player in PlayerReference::all_players(game){
            player.before_initial_role_creation(game);
        }
    }
}