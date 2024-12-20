use crate::game::{
    components::synopsis::SynopsisTracker, player::PlayerReference, win_condition::WinCondition, Game
};

#[must_use = "Event must be invoked"]
pub struct OnConvert{
    player: PlayerReference,
    old: WinCondition,
    new: WinCondition,
}
impl OnConvert{
    pub fn new(player: PlayerReference, old: WinCondition, new: WinCondition) -> Self{
        Self{ player, old, new }
    }
    pub fn invoke(self, game: &mut Game){
        SynopsisTracker::on_convert(game, self.player, self.old, self.new);
    }
}