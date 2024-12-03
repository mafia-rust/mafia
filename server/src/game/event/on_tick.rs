use crate::game::{
    ability_input::saved_ability_inputs::SavedControllersMap, 
    Game
};

pub struct OnTick;

impl OnTick{
    pub fn new()->Self{
        Self{}
    }
    pub fn invoke(&self, game: &mut Game){
        SavedControllersMap::on_tick(game);
    }
}