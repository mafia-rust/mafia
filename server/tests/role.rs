mod kit;
pub(crate) use kit::{assert_contains, assert_not_contains};
use mafia_server::game::chat::{MessageSender, ChatGroup};
// Pub use so that submodules don't have to re-import everything.
pub use mafia_server::game::{role::{RoleState, transporter::Transporter, medium::Medium, jailor::Jailor, vigilante::Vigilante, sheriff::Sheriff, escort::Escort, mafioso::Mafioso, bodyguard::Bodyguard}, phase::PhaseState, chat::ChatMessage};
pub use mafia_server::packet::ToServerPacket;
pub use mafia_server::game::phase::PhaseType;

#[test]
fn medium_recieves_dead_messages_from_jail() {
    kit::scenario!(game in Evening 1 where
        medium: Medium,
        jailor: Jailor,
        townie: Sheriff
    );
    townie.die();
    jailor.day_target(medium);

    game.skip_to(PhaseType::Night, 1);
    let dead_message = "Hello medium!! Are you there!?";
    townie.send_message(dead_message);

    assert_contains!(medium.get_messages(), 
        ChatMessage::Normal { 
            text: dead_message.to_string(),
            message_sender: MessageSender::Player { player: townie.index() },
            chat_group: ChatGroup::Dead
        }
    );
}

#[test]
fn bodyguard_protects() {
    kit::scenario!(game in Night 1 where
        maf: Mafioso,
        bg: Bodyguard,
        townie: Sheriff
    );

    maf.set_night_target(townie);
    bg.set_night_target(townie);

    game.skip_to(PhaseType::Morning, 2);

    assert!(townie.get_messages().contains(&ChatMessage::BodyguardProtectedYou));

    assert!(townie.alive());
    assert!(!bg.alive());
    assert!(!maf.alive());
}

#[test]
fn sheriff_investigates() {
    kit::scenario!(game in Night 1 where
        sher: Sheriff,
        mafia: Mafioso,
        townie: Sheriff
    );
    sher.set_night_targets(vec![mafia]);
    
    game.skip_to(PhaseType::Morning, 2);
    assert_contains!(sher.get_messages(), ChatMessage::SheriffResult { suspicious: true });

    game.skip_to(PhaseType::Night, 2);
    sher.set_night_targets(vec![townie]);
    
    game.skip_to(PhaseType::Morning, 3);
    assert_contains!(sher.get_messages(), ChatMessage::SheriffResult { suspicious: false });
}

/// A module for tests involving transporter interactions.
pub mod transporter_interactions {
    use super::*;

    /// Tests if transporter properly swaps, redirecting actions on their first target to their
    /// second. The vigilante will try to kill town1, which should end up killing town2.
    #[test]
    fn vigilante_kills_transportee() {
        kit::scenario!(game in Night 2 where
            trans: Transporter,
            vigi: Vigilante,
            town1: Sheriff,
            town2: Sheriff
        );
        trans.set_night_targets(vec![town1, town2]);
        vigi.set_night_target(town1);
    
        game.skip_to(PhaseType::Morning, 3);
        assert!(town1.alive());
        assert!(!town2.alive());
        
        game.skip_to(PhaseType::Morning, 4);
        assert!(!vigi.alive());
    }

    /// Test if when the transporter swaps two people, visits to the second target also redirect to
    /// the first target. The escort will try to roleblock town2, which will end up roleblocking 
    /// town1.
    #[test]
    fn escort_roleblocks_reverse_transportee() {
        kit::scenario!(game in Night 1 where 
            trans: Transporter,
            escort: Escort,
            town1: Sheriff,
            town2: Sheriff
        );
        trans.set_night_targets(vec![town1, town2]);
        escort.set_night_target(town2);
    
        game.skip_to(PhaseType::Morning, 2);
        assert!(town1.was_roleblocked());
        assert!(!town2.was_roleblocked());
    }
    
    /// Test that the bodyguard protects the person their target was swapped with
    #[test]
    fn bodyguard_protects_transported_target() {
        kit::scenario!(game in Night 1 where
            trans: Transporter,
            maf: Mafioso,
            bg: Bodyguard,
            t1: Sheriff,
            t2: Sheriff
        );
        trans.set_night_targets(vec![t1, t2]);
        maf.set_night_target(t1);
        bg.set_night_target(t1);
        
        game.skip_to(PhaseType::Morning, 2);
        assert!(t1.alive());
        assert!(t2.alive());
        assert!(trans.alive());
        assert!(!bg.alive());
        assert!(!maf.alive());

        assert_not_contains!(t1.get_messages(), ChatMessage::BodyguardProtectedYou);
        assert_contains!(t2.get_messages(), ChatMessage::BodyguardProtectedYou);
    }
}