extern crate proc_macro;
use proc_macro::TokenStream;

macro_rules! make_role_enum {
    (
        $(
            $name:ident : $file:ident $({
                $($data_ident:ident: $data_type:ty = $data_def:expr),*
            })?
        ),*
    ) => {
        use crate::game::player::PlayerID;
        use crate::game::Game;
        $(mod $file;)*

        #[derive(Clone, Copy)]
        pub enum Role {
            $($name),*
        }

        #[derive(Clone, Copy)]
        pub enum RoleData {
            $($name $({
                $($data_ident: $data_type),*
            })?),*
        }

        impl Role {
            pub fn default_data(&self) -> RoleData {
                match self {
                    $(Role::$name => RoleData::$name$({
                        $($data_ident: $data_def),*
                    })?),*
                }
            }

            pub fn is_suspicious(&self) -> bool {
                match self {
                    $(Role::$name => $file::SUSPICIOUS),*
                }
            }

            pub fn is_witchable(&self) -> bool {
                match self {
                    $(Role::$name => $file::WITCHABLE),*
                }
            }

            pub fn get_defense(&self) -> u8 {
                match self {
                    $(Role::$name => $file::DEFENSE),*
                }
            }

            pub fn is_roleblockable(&self) -> bool {
                match self {
                    $(Role::$name => $file::ROLEBLOCKABLE),*
                }
            }

            pub fn do_night_action(&mut self, source: PlayerID, game: &mut Game) {
                match self {
                    $(Role::$name => $file::do_night_action(source, game)),*
                }
            }
            pub fn do_day_action(&mut self, source: PlayerID, game: &mut Game) {
                match self {
                    $(Role::$name => $file::do_day_action(source, game)),*
                }
            }
            pub fn can_night_target(&self, source: PlayerID, target: PlayerID, game: &Game) -> bool {
                match self {
                    $(Role::$name => $file::can_night_target(source, target, game)),*
                }
            }
            pub fn can_day_target(&self, source: PlayerID, target: PlayerID, game: &Game) -> bool {
                match self {
                    $(Role::$name => $file::can_day_target(source, target, game)),*
                }
            }
        }

        impl RoleData {
            pub fn role(&self) -> Role {
                match self {
                    $(RoleData::$name$({
                        $($data_ident: _),*
                    })? => Role::$name),*
                }
            }
        }
    }
}

macro_rules! create_role {
    (
        $name:ident

        $(let $item_name:ident: $item_value:expr;)*

        $(fn $func:ident($($arg:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block)*
    ) => {
        use crate::game::player::{PlayerID, Player};
        use crate::game::visit::Visit;
        use crate::game::Game;

        $(parse_role_item! {$item_name: $item_value})*

        $(parse_role_function! {$func($($arg),*) $body})*
    };
}

macro_rules! parse_role_item {
    (defense: $defense:expr) => {
        pub(super) const DEFENSE: u8 = $defense;
    };
    (roleblockable: $roleblockable:expr) => {
        pub(super) const ROLEBLOCKABLE: bool = $roleblockable;
    };
    (witchable: $witchable:expr) => {
        pub(super) const WITCHABLE: bool = $witchable;
    };
    (suspicious: $sus:expr) => {
        pub(super) const SUSPICIOUS: bool = $sus;
    };
}

macro_rules! parse_role_function {
    (
        do_night_action($actor:ident, $game:ident) 
            $do_night_action:block
    ) => {
        pub(super) fn do_night_action(actor: PlayerID, $game: &mut Game) {
            let $actor = $game.get_player_mut(actor);
            $do_night_action
        }
    };
    (
        can_night_target($actor:ident, $target:ident, $game:ident)
            $can_target:block
    ) => {
        pub(super) fn can_night_target(actor: PlayerID, target: PlayerID, $game: &Game) -> bool {
            let $actor = $game.get_player(actor);
            let $target = $game.get_player(target);
            $can_target
        }
    };
    (
        do_day_action($actor:ident, $target:ident, $game:ident) 
            $do_day_action:block
    ) => {
        pub(super) fn do_day_action(actor: PlayerID, $game: &mut Game) {
            let $actor = $game.get_player_mut(actor);
            let $target = todo!();
            $do_day_action
        }
    };
    (
        can_day_target($actor:ident, $target:ident, $game:ident)
            $can_day_target:block
    ) => {
        pub(super) fn can_day_target(actor: PlayerID, target: PlayerID, $game: &Game) -> bool {
            let $actor = $game.get_player(actor);
            let $target = $game.get_player(target);
            $can_day_target
        }
    };
    (
        convert_targets_to_visits($targets:ident, $game:ident)
            $convert_targets_to_visits:block
    ) => {
        pub(super) fn convert_targets_to_visits($targets: &[PlayerID], $game: &Game) -> Vec<Visit>
            $convert_targets_to_visits
    };
}