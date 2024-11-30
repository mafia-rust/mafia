use crate::game::{
    ability_input::saved_ability_inputs::AllPlayersSavedAbilityInputs, 
    Game
};

pub struct OnTick;

impl OnTick{
    pub fn new()->Self{
        Self{}
    }
    pub fn invoke(&self, game: &mut Game){
        AllPlayersSavedAbilityInputs::on_tick(game);
    }
}