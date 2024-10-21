pub mod obscured_graves;
pub mod random_love_links;
pub mod dead_can_chat;
pub mod no_abstaining;
pub mod no_death_cause;
pub mod no_trials;
pub mod elector_and_president;

use std::collections::HashSet;

use dead_can_chat::DeadCanChat;
use no_abstaining::NoAbstaining;
use no_trials::NoTrials;
use obscured_graves::ObscuredGraves;
use elector_and_president::ElectorAndPresident;
use random_love_links::RandomLoveLinks;
use no_death_cause::NoDeathCause;
use serde::{Deserialize, Serialize};

use super::{grave::GraveReference, phase::PhaseType, Game};


#[enum_delegate::register]
pub trait ModifierTrait where Self: Clone + Sized{
    fn modifier_type(&self)->ModifierType;
    fn on_grave_added(self, _game: &mut Game, _event: GraveReference) {}
    fn on_game_start(self, _game: &mut Game) {}
    fn on_phase_start(self, _game: &mut Game, _phase: PhaseType) {}
}

#[enum_delegate::implement(ModifierTrait)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ModifierState{
    ObscuredGraves(ObscuredGraves),
    RandomLoveLinks(RandomLoveLinks),
    DeadCanChat(DeadCanChat),
    NoAbstaining(NoAbstaining),
    NoDeathCause(NoDeathCause),
    NoTrials(NoTrials),
    ElectorAndPresident(ElectorAndPresident),
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModifierType{
    ObscuredGraves,
    RandomLoveLinks,
    DeadCanChat,
    NoAbstaining,
    NoDeathCause,
    NoTrials,
    ElectorAndPresident,
}
impl ModifierType{
    pub fn default_state(&self)->ModifierState{
        match self{
            Self::ObscuredGraves => ModifierState::ObscuredGraves(ObscuredGraves::default()),
            Self::RandomLoveLinks => ModifierState::RandomLoveLinks(RandomLoveLinks::default()),
            Self::DeadCanChat => ModifierState::DeadCanChat(DeadCanChat::default()),
            Self::NoAbstaining => ModifierState::NoAbstaining(NoAbstaining::default()),
            Self::NoDeathCause => ModifierState::NoDeathCause(NoDeathCause::default()),
            Self::NoTrials => ModifierState::NoTrials(NoTrials::default()),
            Self::ElectorAndPresident => ModifierState::ElectorAndPresident(ElectorAndPresident::default()),
        }
    }
}

#[derive(Default)]
pub struct Modifiers{
    modifiers: HashSet<ModifierState>,
}

impl Modifiers{
    pub fn modifier_is_enabled(game: &Game, modifier: ModifierType)->bool{
        game.modifiers.modifiers.iter().any(|m| m.modifier_type() == modifier)
    }
    pub fn from_settings(modifiers: HashSet<ModifierType>)->Self{
        let modifiers = modifiers.into_iter().map(|m| m.default_state()).collect();
        Self{
            modifiers,
        }
    }
    pub fn set_modifier_state(game: &mut Game, state: ModifierState) {
        game.modifiers.modifiers.retain(|m| m.modifier_type() != state.modifier_type());
        game.modifiers.modifiers.insert(state);
    }
    pub fn get_modifier_state(game: &Game, modifier_type: ModifierType) -> Option<ModifierState> {
        game.modifiers.modifiers.iter().find(|m| m.modifier_type() == modifier_type).cloned()
    }
    pub fn on_grave_added(game: &mut Game, event: GraveReference){
        for modifier in game.modifiers.modifiers.iter().cloned().collect::<Vec<_>>(){
            modifier.on_grave_added(game, event);
        }
    }
    pub fn on_game_start(game: &mut Game){
        for modifier in game.modifiers.modifiers.iter().cloned().collect::<Vec<_>>(){
            modifier.on_game_start(game);
        }
    }
    pub fn on_phase_start(game: &mut Game, phase: PhaseType) {
        for modifier in game.modifiers.modifiers.iter().cloned().collect::<Vec<_>>(){
            modifier.on_phase_start(game, phase);
        }
    }
}