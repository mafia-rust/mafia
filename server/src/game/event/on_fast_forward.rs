use crate::game::Game;

#[must_use = "Event must be invoked"]
pub struct OnFastForward;
impl OnFastForward{
    pub fn invoke(game: &mut Game){
        game.on_fast_forward();
    }
}