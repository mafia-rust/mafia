use crate::game::{
    ability_input::saved_ability_inputs::SavedAbilityInputs, 
    Game
};

pub struct OnTick;

impl OnTick{
    pub fn new()->Self{
        Self{}
    }
    pub fn invoke(&self, game: &mut Game){
        SavedAbilityInputs::on_tick(game);
    }
}