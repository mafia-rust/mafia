use super::Game;

pub mod on_any_death;
pub mod on_fast_forward;
pub mod on_game_ending;
pub mod on_phase_start;
pub mod on_grave_added;
pub mod on_game_start;
pub mod on_role_switch;
pub mod on_convert;
pub mod before_role_switch;
pub mod before_phase_end;
pub mod on_night_priority;
pub mod on_remove_role_label;
pub mod before_initial_role_creation;
pub mod on_ability_input_received;
pub mod on_validated_ability_input_received;
pub mod on_controller_selection_changed;
pub mod on_tick;
pub mod on_player_roleblocked;
pub mod on_visit_wardblocked;
pub mod on_whisper;


pub trait EventPriority: Sized + Copy {
    fn first() -> Self;
    fn next(self) -> Option<Self>;
}

pub trait Event: Sized {
    type FoldValue;
    type Inner;
    type Priority: EventPriority;

    fn listeners() -> Vec<EventListenerFunction<Self>>;
    fn initial_fold_value(&self) -> Self::FoldValue;
    fn inner(&self) -> Self::Inner;
    fn invoke(self, game: &mut Game) {
        let mut priority = Self::Priority::first();
        let mut fold = self.initial_fold_value();

        loop {
            for listener in Self::listeners() {
                listener(game, &self.inner(), &mut fold, priority);
            }

            priority = match priority.next() {
                Some(p) => p,
                None => break,
            };
        }
    }
}

#[expect(type_alias_bounds, reason="This is fine")]
pub type EventListenerFunction<E: Event> = fn(&mut Game, &E::Inner, &mut E::FoldValue, E::Priority);

impl EventPriority for () {
    fn first() -> Self {
    }
    fn next(self) -> Option<Self> {
        None
    }
}
