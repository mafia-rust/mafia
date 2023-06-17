use mafia_server::game::{player::PlayerReference, Game, role::RoleState, role_list::RoleListEntry, settings::Settings, test::mock_game};

pub mod time;
pub mod player;

pub struct TestScenario {
    pub game: Game,
    pub players: Vec<PlayerReference>
}

#[allow(unused)]
macro_rules! scenario {
    ($game:ident in Evening 1 $($tok:tt)*) => {
        kit::scenario!($game in Evening 1 $($tok)*);
    };
    ($game:ident in $phase:ident $day:literal $($tok:tt)*) => {
        kit::scenario!($game $($tok)*);
        kit::time::advance_phase($game); // Night 1
        kit::time::skip_days($game, $day - 1); // Night $day
        kit::time::skip_to_phase($game, PhaseType::$phase);
    };
    ($game:ident where
        $(ref $name:ident: $role:ident),*
    ) => {
        let mut scenario = kit::_init::create_basic_scenario(
            vec![$(RoleState::$role($role::default())),*]
        );

        let $game = &mut scenario.game;

        let players: Vec<kit::player::TestPlayer> = scenario.players
            .into_iter()
            .map(|player| kit::player::TestPlayer::new(player, &$game))
            .collect();

        let [$($name),*] = players.as_slice() else {unreachable!()};
        $(let $name = *$name;)*
    }
}

#[allow(unused)]
pub(crate) use scenario;

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
        
        for (index, role) in roles.into_iter().enumerate() {
            let player = PlayerReference::new(&game, index as u8).unwrap();
            player.set_role(&mut game, role);
            players.push(player);
        }
    
        TestScenario { game, players }
    }
}
