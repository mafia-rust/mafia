use mafia_server::game::{player::PlayerReference, Game, role::RoleState, role_list::RoleListEntry, settings::Settings, test::mock_game};


pub struct TestScenario {
    pub game: Game,
    pub players: Vec<PlayerReference>
}

#[allow(unused)]
macro_rules! init_test {
    ($game:ident, 
        $($name:ident @ $role:ident),*
    ) => {
        let mut scenario = common::__init_test(vec![$(RoleState::$role($role::default())),*]);

        let $game = &mut scenario.game;
        let [$($name),*] = scenario.players.as_slice() else {unreachable!()};
        $(
            let $name = *$name;
        )*
    }
}

pub fn __init_test(roles: Vec<RoleState>) -> TestScenario {
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

#[allow(unused)]
pub(crate) use init_test;