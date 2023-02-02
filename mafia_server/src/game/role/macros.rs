
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

        defense: $defense:expr;
        roleblockable: $roleblockable:expr;
        witchable: $witchable:expr;
        suspicious: $sus:expr;

        fn do_night_action(actor: &mut Player, game: &mut Game) 
            $do_night_action:block

        fn can_night_target(actor: &Player, target: &Player, game: &Game) -> bool
            $can_target:block

        fn do_day_action(actor: &mut Player, target: &mut Player, game: &mut Game) 
            $do_day_action:block

        fn can_day_target(actor: &Player, target: &Player, game: &Game) -> bool
            $can_day_target:block

        fn convert_targets_to_visits(targets: &Vec<PlayerID>, game: &Game) -> Vec<Visit>
            $convert_targets_to_visits:block
    ) => {
        use crate::game::player::{PlayerID, Player};
        use crate::game::Game;

        pub(super) const SUSPICIOUS: bool = $sus;
        pub(super) const WITCHABLE: bool = $witchable;
        pub(super) const DEFENSE: u8 = $defense;
        pub(super) const ROLEBLOCKABLE: bool = $roleblockable;

        pub(super) fn do_night_action(actor: PlayerID, game: &mut Game) {
            let actor = game.get_player_mut(actor);
            $do_night_action
        }

        pub(super) fn do_day_action(actor: PlayerID, game: &mut Game) {
            let actor = game.get_player_mut(actor);
            $do_day_action
        }

        pub(super) fn can_night_target(actor: PlayerID, target: PlayerID, game: &Game) -> bool {
            let actor = game.get_player(actor);
            let target = game.get_player(target);
            let phase = game.get_current_phase();
            $can_target
        }
        
        pub(super) fn can_day_target(actor: PlayerID, target: PlayerID, game: &Game) -> bool {
            let actor = game.get_player(actor);
            let target = game.get_player(target);
            let phase = game.get_current_phase();
            $can_target
        }

        pub(super) fn convert_targets_to_visits(targets: &Vec<PlayerID>, game: &Game) /*-> Vec<Visit>*/
            $convert_targets_to_visits
    };
}