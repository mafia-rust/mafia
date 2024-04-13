use crate::game::{grave::Grave, Game};


pub struct OnGraveAdded{
    grave: Grave
}
impl OnGraveAdded{
    pub fn new(grave: Grave) -> Self{
        Self{ grave }
    }
    pub fn invoke(self, game: &mut Game){
        game.on_grave_added(self.grave);
    }
}