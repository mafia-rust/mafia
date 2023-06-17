use mafia_server::game::{
    player::PlayerReference, 
    Game, role::RoleState, role_list::RoleListEntry, settings::Settings, 
    test::mock_game
};
pub use mafia_server::{game::{role::{jailor::Jailor, medium::Medium, sheriff::Sheriff, bodyguard::Bodyguard, mafioso::Mafioso, transporter::Transporter, lookout::Lookout, escort::Escort, vigilante::Vigilante}, chat::ChatMessage, phase::PhaseState}, packet::ToServerPacket};


pub struct TestScenario {
    pub game: Game,
    pub players: Vec<PlayerReference>
}

#[cfg(test)]
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
macro_rules! assert_contains {
    ($container:expr, $value:expr) => {
        assert!($container.contains(&$value), "{}", format!("Expected {:?} to contain {:?}", $container, $value));
    };
}
macro_rules! assert_not_contains {
    ($container:expr, $value:expr) => {
        assert!(!$container.contains(&$value), "{}", format!("Expected {:?} not to contain {:?}", $container, $value));
    };
}

macro_rules! init_test {
    ($game:ident,
        $($name:ident @ $role:ident),*
    ) => {
        let mut scenario = __init_test(vec![$(RoleState::$role($role::default())),*]);

        let $game = &mut scenario.game;
        let [$($name),*] = scenario.players.as_slice() else {unreachable!()};
        $(
            let $name = *$name;
        )*
    }
}

// #[cfg(test)]
// mod tests {


    #[test]
    /// For this test, we're seeing if transporter properly swaps.
    /// The vigilante will try to kill town1, which *should* end up killing town2.
    /// Likewise, the escort will try to roleblock town2, which *should* end up roleblocking town1.
    fn vigilante_escort_transported_townie() {
        init_test!(game,
            trans @ Transporter,
            vigi @ Vigilante,
            escort @ Escort,
            lookout @ Lookout,
            town1 @ Sheriff,
            town2 @ Sheriff
        );
        //Evening
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Night
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Morning
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Discussion
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Voting
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Night

        trans.set_chosen_targets(game, vec![town1, town2]);
        vigi.set_chosen_targets(game, vec![town1]);
        lookout.set_chosen_targets(game, vec![town1]);
        escort.set_chosen_targets(game, vec![town2]);

        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Morning
        
        // Trasporrtation works (esc)
        assert!(town1.night_roleblocked(game));
        assert!(!town2.night_roleblocked(game));

        // Transportation works (vigi)
        assert!(town1.alive(game));
        assert!(!town2.alive(game));

        //Lookout sees transport properly
        assert_contains!(
            lookout.deref(game).chat_messages.iter().filter(|m|if let ChatMessage::LookoutResult { players: _ } = m{true}else{false}).collect::<Vec<&ChatMessage>>(),
            &ChatMessage::LookoutResult{players: vec![trans.index(), vigi.index()] }
        );
        
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Discussion
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Voting
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Night
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Morning

        //Vigilante did suicide
        assert!(!vigi.alive(game));
    }

    #[test]
    /// For this test, we're seeing if transporter properly swaps.
    /// and the bodyguard properly swaps protecting transported target
    fn bodyguard_transported_target() {
        init_test!(game,
            trans @ Transporter,
            maf @ Mafioso,
            bg @ Bodyguard,
            t1 @ Sheriff,
            t2 @ Sheriff
        );
        //Evening
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Night

        trans.set_chosen_targets(game, vec![t1, t2]);
        maf.set_chosen_targets(game, vec![t1]);
        bg.set_chosen_targets(game, vec![t1]);
        
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Morning

        assert!(t1.alive(game));
        assert!(t2.alive(game));
        assert!(trans.alive(game));
        assert!(!bg.alive(game));
        assert!(!maf.alive(game));

        assert!(t2.deref(game).chat_messages.contains(&ChatMessage::BodyguardProtectedYou))
    }

    #[test]
    /// For this test, we're seeing if bodyguard works in its most basic sense, and also testing the sheriff
    fn bodyguard_protect() {
        init_test!(game,
            maf @ Mafioso,
            bg @ Bodyguard,
            sher @ Sheriff,
            lo1 @ Lookout,
            lo2 @ Lookout
        );
        //Evening
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Night

        maf.set_chosen_targets(game, vec![sher]);
        bg.set_chosen_targets(game, vec![sher]);
        sher.set_chosen_targets(game, vec![maf]);
        lo1.set_chosen_targets(game, vec![sher]);
        lo2.set_chosen_targets(game, vec![bg]);
        
        let next_phase = PhaseState::end(game);
        game.start_phase(next_phase);
        //Morning

        assert!(sher.alive(game));
        assert!(!bg.alive(game));
        assert!(!maf.alive(game));
        assert!(lo1.alive(game));
        assert!(lo2.alive(game));

        assert_contains!(sher.deref(game).chat_messages, &ChatMessage::BodyguardProtectedYou);
        assert_contains!(sher.deref(game).chat_messages, &ChatMessage::SheriffResult { suspicious: true });
        assert_contains!(
            lo1.deref(game).chat_messages.iter().filter(|m|if let ChatMessage::LookoutResult { players: _ } = m{true}else{false}).collect::<Vec<&ChatMessage>>(), 
            &ChatMessage::LookoutResult { players: vec![bg.index()] }
        );
        assert_contains!(
            lo2.deref(game).chat_messages.iter().filter(|m|if let ChatMessage::LookoutResult { players: _ } = m{true}else{false}).collect::<Vec<&ChatMessage>>(), 
            &ChatMessage::LookoutResult { players: vec![maf.index()] }
        );
    }

    #[test]
    fn medium_recieves_dead_messages_from_jail() {
        init_test!(game, 
            medium @ Medium,
            jailor @ Jailor,
            townie @ Sheriff
        );

        townie.set_alive(game, false);

        game.on_client_message(jailor.index(), ToServerPacket::DayTarget { player_index: medium.index() });

        game.start_phase(PhaseState::Night);

        let dead_message = "Hello medium!! Are you there!?".to_string();
        game.on_client_message(townie.index(), ToServerPacket::SendMessage { text: dead_message.clone() });

        let last_recieved_message = match medium.deref(game).chat_messages.last() {
            Some(ChatMessage::Normal { text, .. }) => {
                text.clone()
            }
            _ => panic!("No messages have been received!")
        };

        assert_eq!(dead_message, last_recieved_message);
    }
// }