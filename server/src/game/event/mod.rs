use super::Game;

pub(super) mod on_any_death;
pub(super) mod on_fast_forward;
pub(super) mod on_game_ending;
pub(super) mod on_phase_start;
pub(super) mod on_grave_added;
pub(super) mod on_game_start;
pub(super) mod on_role_switch;
pub(super) mod on_convert;
pub(super) mod before_role_switch;
pub(super) mod before_phase_end;
pub(super) mod on_midnight;
pub(super) mod on_remove_role_label;
pub(super) mod before_initial_role_creation;
pub(super) mod on_ability_input_received;
pub(super) mod on_validated_ability_input_received;
pub(super) mod on_controller_selection_changed;
pub(super) mod on_tick;
pub(super) mod on_player_roleblocked;
pub(super) mod on_visit_wardblocked;
pub(super) mod on_whisper;


pub trait EventPriority: Sized + Copy {
    fn values() -> Vec<Self>;
}

///
/// 
/// 
/// // Event listener type
/// // pub type EventListenerFunction<E: Event> = fn(&mut Game, &E, &mut E::FoldValue, E::Priority);
/// 
pub trait Event: Sized {
    type FoldValue;
    type Priority: EventPriority;

    fn listeners() -> Vec<EventListenerFunction<Self>>;
    fn initial_fold_value(&self) -> Self::FoldValue;
    fn invoke(self, game: &mut Game) {
        let mut fold = self.initial_fold_value();

        for priority in Self::Priority::values() {
            for listener in Self::listeners() {
                listener(game, &self, &mut fold, priority);
            }
        }
    }
}

#[expect(type_alias_bounds, reason="This is fine")]
pub type EventListenerFunction<E: Event> = fn(&mut Game, &E, &mut E::FoldValue, E::Priority);

impl EventPriority for () {
    fn values() -> Vec<Self> {vec![()]}
}




#[macro_export]
macro_rules! event_priority {
    (
        $name:ident{
            $($variant:ident),*
        }
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),*
        }
        impl $crate::game::event::EventPriority for $name {
            fn values() -> Vec<Self> {
                vec![$(Self::$variant),*]
            }
        }
    };
}