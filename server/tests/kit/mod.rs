use mafia_server::game::{player::PlayerReference, Game, role::RoleState, role_list::RoleListEntry, settings::Settings, test::mock_game};

pub mod player;
pub mod game;

pub struct TestScenario {
    pub game: Game,
    pub players: Vec<PlayerReference>
}

#[allow(unused)]
macro_rules! scenario {
    ($game:ident in Evening 1 $($tok:tt)*) => {
        kit::scenario!($game $($tok)*);
    };
    ($game:ident in $phase:ident $day:literal $($tok:tt)*) => {
        kit::scenario!($game $($tok)*);
        $game.skip_to(PhaseType::$phase, $day);
    };
    ($game:ident where
        $($name:ident: $role:ident),*
    ) => {
        let mut scenario = kit::_init::create_basic_scenario(
            vec![$(RoleState::$role($role::default())),*]
        );

        let game = &mut scenario.game;

        let players: Vec<kit::player::TestPlayer> = scenario.players
            .into_iter()
            .map(|player| kit::player::TestPlayer::new(player, &game))
            .collect();

        let [$($name),*] = players.as_slice() else {unreachable!()};

        let mut $game = kit::game::TestGame::new(game);
        $(let $name = *$name;)*
    }
}

#[allow(unused)]
macro_rules! assert_contains {
    ($container:expr, $value:expr) => {
        assert!($container.contains(&$value), "{}", format!("Expected {:#?} to contain {:?}", $container, $value));
    };
}
#[allow(unused)]
macro_rules! assert_not_contains {
    ($container:expr, $value:expr) => {
        assert!(!$container.contains(&$value), "{}", format!("Expected {:#?} not to contain {:?}", $container, $value));
    };
}

#[allow(unused)]
pub(crate) use {scenario, assert_contains, assert_not_contains};

/// Stuff that shouldn't be called directly - only in macro invocations.
#[doc(hidden)]
pub mod _init {
    use super::*;

    pub fn create_basic_scenario(roles: Vec<RoleState>) -> TestScenario {
        let mut role_list = Vec::new();
        for _ in 0..roles.len() {
            role_list.push(RoleListEntry::Any);
        }
    
        let mut game = mock_game(Settings {
            role_list,
            ..Default::default()
        }, roles.len());
    
        let mut players = Vec::new();
        
        // Assign role state manually before setting role, so on_role_creation knows the correct other roles in the game.
        // (set_role calls on_role_creation)
        for (index, role) in roles.iter().enumerate() {
            let player = PlayerReference::new(&game, index as u8).unwrap();
            player.set_role_state(&mut game, role.clone());
        }

        for (index, role) in roles.into_iter().enumerate() {
            let player = PlayerReference::new(&game, index as u8).unwrap();
            player.set_role(&mut game, role);
            players.push(player);
        }
    
        TestScenario { game, players }
    }
}
