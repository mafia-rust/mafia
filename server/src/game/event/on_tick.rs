use crate::game::{components::generic_ability::GenericAbilitySaveComponent, Game};

pub struct OnTick;

impl OnTick{
    pub fn new()->Self{
        Self{}
    }
    pub fn invoke(&self, game: &mut Game){
        GenericAbilitySaveComponent::on_tick(game);
    }
}