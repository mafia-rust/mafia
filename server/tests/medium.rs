mod common;

use mafia_server::{game::{phase::PhaseState, chat::ChatMessage, role::{RoleState, medium::Medium, jailor::Jailor, sheriff::Sheriff}}, packet::ToServerPacket};

#[test]
fn medium_recieves_dead_messages_from_jail() {
    common::init_test!(game, 
        medium @ Medium,
        jailor @ Jailor,
        townie @ Sheriff
    );

    townie.set_alive(game, false);

    jailor.set_chosen_targets(game, vec![medium]);

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
