use std::collections::HashSet;

use obscured_graves::ObscuredGraves;
use serde::{Deserialize, Serialize};

use super::{grave::GraveReference, Game};

pub mod obscured_graves;

#[enum_delegate::register]
pub trait ModifierTrait{
    // fn on_game_start(&self, game: &mut Game, event: OnGameStart);
    fn modifier_type(&self)->ModifierType;
    fn on_grave_added(self, game: &mut Game, event: GraveReference);
}

#[enum_delegate::implement(ModifierTrait)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ModifierState{
    ObscuredGraves(ObscuredGraves),
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModifierType{
    ObscuredGraves,
}
impl ModifierType{
    pub fn default_state(&self)->ModifierState{
        match self{
            Self::ObscuredGraves => ModifierState::ObscuredGraves(ObscuredGraves::default()),
        }
    }
}

#[derive(Default)]
pub struct Modifiers{
    modifiers: HashSet<ModifierState>,
}

impl Modifiers{
    pub fn from_settings(modifiers: HashSet<ModifierType>)->Self{
        let modifiers = modifiers.into_iter().map(|m| m.default_state()).collect();
        Self{
            modifiers,
        }
    }
    pub fn on_grave_added(game: &mut Game, event: GraveReference){
        for modifier in game.modifiers.modifiers.iter().map(|m| m.clone()).collect::<Vec<_>>(){
            modifier.on_grave_added(game, event);
        }
    }
}