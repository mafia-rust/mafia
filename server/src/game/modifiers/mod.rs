pub mod obscured_graves;
pub mod random_love_links;
pub mod dead_can_chat;
pub mod no_abstaining;
pub mod no_death_cause;
pub mod mafia_hit_orders;

use dead_can_chat::DeadCanChat;
use no_abstaining::NoAbstaining;
use obscured_graves::ObscuredGraves;
use random_love_links::RandomLoveLinks;
use no_death_cause::NoDeathCause;
use mafia_hit_orders::MafiaHitOrders;
use serde::{Deserialize, Serialize};

use crate::{vec_map::VecMap, vec_set::VecSet};

use super::{grave::GraveReference, Game};


#[enum_delegate::register]
pub trait ModifierTrait where Self: Clone + Sized{
    fn on_ability_input_received(self, _game: &mut Game, _actor_ref: crate::game::player::PlayerReference, _input: crate::game::ability_input::AbilityInput) {}
    fn on_night_priority(self, _game: &mut Game, _priority: crate::game::role::Priority) {}
    fn before_phase_end(self, _game: &mut Game, _phase: super::phase::PhaseType) {}
    fn on_grave_added(self, _game: &mut Game, _event: GraveReference) {}
    fn on_game_start(self, _game: &mut Game) {}
    fn on_any_death(self, _game: &mut Game, _player: crate::game::player::PlayerReference) {}
    fn before_initial_role_creation(self, _game: &mut Game) {}
}

#[enum_delegate::implement(ModifierTrait)]
#[derive(Clone, PartialEq, Eq)]
pub enum ModifierState{
    ObscuredGraves(ObscuredGraves),
    RandomLoveLinks(RandomLoveLinks),
    DeadCanChat(DeadCanChat),
    NoAbstaining(NoAbstaining),
    NoDeathCause(NoDeathCause),
    MafiaHitOrders(MafiaHitOrders),
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModifierType{
    ObscuredGraves,
    RandomLoveLinks,
    DeadCanChat,
    NoAbstaining,
    NoDeathCause,
    MafiaHitOrders,
}
impl ModifierType{
    pub fn default_state(&self)->ModifierState{
        match self{
            Self::ObscuredGraves => ModifierState::ObscuredGraves(ObscuredGraves::default()),
            Self::RandomLoveLinks => ModifierState::RandomLoveLinks(RandomLoveLinks::default()),
            Self::DeadCanChat => ModifierState::DeadCanChat(DeadCanChat::default()),
            Self::NoAbstaining => ModifierState::NoAbstaining(NoAbstaining::default()),
            Self::NoDeathCause => ModifierState::NoDeathCause(NoDeathCause::default()),
            Self::MafiaHitOrders => ModifierState::MafiaHitOrders(MafiaHitOrders::default()),
        }
    }
}
impl From<&ModifierState> for ModifierType{
    fn from(state: &ModifierState)->Self{
        match state {
            ModifierState::ObscuredGraves(_) => Self::ObscuredGraves,
            ModifierState::RandomLoveLinks(_) => Self::RandomLoveLinks,
            ModifierState::DeadCanChat(_) => Self::DeadCanChat,
            ModifierState::NoAbstaining(_) => Self::NoAbstaining,
            ModifierState::NoDeathCause(_) => Self::NoDeathCause,
            ModifierState::MafiaHitOrders(_) => Self::MafiaHitOrders,
        }
    }
}




#[derive(Default)]
pub struct Modifiers{
    modifiers: VecMap<ModifierType, ModifierState>,
}

impl Modifiers{
    pub fn modifier_is_enabled(game: &Game, modifier: ModifierType)->bool{
        game.modifiers.modifiers.contains(&modifier)
    }
    pub fn get_modifier(game: &Game, modifier: ModifierType)->Option<&ModifierState>{
        game.modifiers.modifiers.get(&modifier)
    }
    pub fn get_modifier_inner<'a, T>(game: &'a Game, modifier: ModifierType)->Option<&'a T>
    where 
        // T: ModifierTrait,
        // ModifierState: TryInto<T>,
        &'a ModifierState: TryInto<&'a T>,
    {
        game.modifiers.modifiers.get(&modifier).map(|s|
            s.try_into().ok()
        )
        .flatten()
    }
    pub fn set_modifier(game: &mut Game, state: ModifierState){
        game.modifiers.modifiers.insert(
            <&ModifierState as Into<ModifierType>>::into(&state).clone(),
            state
        );
    }
    pub fn default_from_settings(modifiers: VecSet<ModifierType>)->Self{
        let modifiers = modifiers
            .into_iter().map(|m|{let state = m.default_state(); (m, state)}).collect();
        Self{
            modifiers,
        }
    }
    pub fn on_night_priority(game: &mut Game, priority: crate::game::role::Priority){
        for modifier in game.modifiers.modifiers.clone(){
            modifier.1.on_night_priority(game, priority);
        }
    }
    pub fn on_ability_input_received(game: &mut Game, actor_ref: crate::game::player::PlayerReference, input: crate::game::ability_input::AbilityInput){
        for modifier in game.modifiers.modifiers.clone(){
            modifier.1.on_ability_input_received(game, actor_ref, input.clone());
        }
    }
    pub fn on_grave_added(game: &mut Game, event: GraveReference){
        for modifier in game.modifiers.modifiers.clone(){
            modifier.1.on_grave_added(game, event);
        }
    }
    pub fn on_game_start(game: &mut Game){
        for modifier in game.modifiers.modifiers.clone(){
            modifier.1.on_game_start(game);
        }
    }
    pub fn before_phase_end(game: &mut Game, phase: super::phase::PhaseType){
        for modifier in game.modifiers.modifiers.clone(){
            modifier.1.before_phase_end(game, phase);
        }
    }
    pub fn on_any_death(game: &mut Game, player: crate::game::player::PlayerReference){
        for modifier in game.modifiers.modifiers.clone(){
            modifier.1.on_any_death(game, player);
        }
    }
    pub fn before_initial_role_creation(game: &mut Game){
        for modifier in game.modifiers.modifiers.clone(){
            modifier.1.before_initial_role_creation(game);
        }
    }
}