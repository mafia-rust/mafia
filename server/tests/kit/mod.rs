use mafia_server::game::{
    chat::ChatMessageVariant, 
    player::PlayerReference, 
    role::RoleState, 
    settings::Settings, 
    test::mock_game, 
    Game
};

pub mod player;
pub mod game;

pub struct TestScenario {
    pub game: Game,
    pub players: Vec<PlayerReference>
}

#[allow(unused)]
macro_rules! scenario {
    ($game:ident in Briefing 1 $($tok:tt)*) => {
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

//Formats messages in a way where it's clear which phase each message was sent in
pub fn _format_messages_debug(messages: Vec<ChatMessageVariant>) -> String{
    let mut string = "[\n".to_string();

    for message in messages {
        string += match message {
            ChatMessageVariant::PhaseChange{..} => "\t",
            _ => "\t\t",
        };
        string += format!("{:?}", message).as_str();
        string += "\n";
    }
    string += "]";
    string
}

/// Stuff that shouldn't be called directly - only in macro invocations.
#[doc(hidden)]
pub mod _init {
    use mafia_server::game::{role::Role, role_list::{RoleList, RoleOutline, RoleOutlineOption, RoleOutlineOptionInsiderGroups, RoleOutlineOptionRoles, RoleOutlineOptionWinCondition}};
    use vec1::vec1;

    use super::*;

    pub fn create_basic_scenario(roles: Vec<RoleState>) -> TestScenario {
        let mut role_list = Vec::new();
        for role in roles.iter() {
            role_list.push(RoleOutline { options: 
                vec1![RoleOutlineOption {
                    roles: RoleOutlineOptionRoles::Role { role: role.role() },
                    insider_groups: RoleOutlineOptionInsiderGroups::RoleDefault,
                    win_condition: RoleOutlineOptionWinCondition::RoleDefault,
                }]
            });
        }
    
        let game = match mock_game(Settings {
            role_list: RoleList(role_list),
            enabled_roles: Role::values().into_iter().collect(),
            ..Default::default()
        }, roles.len() as u8){
            Ok(game) => game,
            Err(err) => panic!("Failed to create game: {:?}", err),
        };
    
        let players = PlayerReference::all_players(&game).collect();
    
        TestScenario { game, players }
    }
}
