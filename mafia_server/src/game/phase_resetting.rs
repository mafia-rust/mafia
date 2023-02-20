use std::{ops::{Deref, DerefMut}};
use crate::game::phase::PhaseType;
use super::{player::Player, Game};

type ResetFunction<T> = dyn Fn(Game) -> T;

pub struct PhaseResetting<T: Clone + Default> {
    value: T,
    reset: *const ResetFunction<T>, // This makes it so that Game and Player can't be sent across threads
                                    // But we don't really need that anyway.
    phase: PhaseType
}

impl<'a, T: Clone + Default> PhaseResetting<T> {
    pub fn new(phase: PhaseType, reset: &ResetFunction<T>) -> Self {
        PhaseResetting { 
            value: T::default(), // This will get changed immediately anyway
            reset, 
            phase
        }
    }

    pub fn get(&self) -> T {
        self.value.clone()
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn reset(&mut self, game: Game) {
        unsafe { self.value = (*(self.reset))(game); }
    }
}

impl<'a, T: Clone + Default> Deref for PhaseResetting<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T: Clone + Default> DerefMut for PhaseResetting<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}