mod kit;
pub use kit::time as time_kit;
// Pub use so that submodules don't have to re-import everything.
pub use mafia_server::game::{role::{RoleState, transporter::Transporter, medium::Medium, jailor::Jailor, vigilante::Vigilante, sheriff::Sheriff, escort::Escort, mafioso::Mafioso, bodyguard::Bodyguard}, phase::PhaseState, chat::ChatMessage};
pub use mafia_server::packet::ToServerPacket;
pub use mafia_server::game::phase::PhaseType;

#[test]
fn medium_recieves_dead_messages_from_jail() {
    kit::scenario!(game where
        ref medium: Medium,
        ref jailor: Jailor,
        ref townie: Sheriff
    );

    townie.die();

    jailor.day_target(medium);

    game.start_phase(PhaseState::Night);

    let dead_message = "Hello medium!! Are you there!?";
    townie.send_message(dead_message);

    let last_recieved_message = match medium.get_messages().last() {
        Some(ChatMessage::Normal { text, .. }) => {
            text.clone()
        }
        _ => panic!("No messages have been received!")
    };

    assert_eq!(dead_message, last_recieved_message);
}

#[test]
fn bodyguard_protects() {
    kit::scenario!(game in Night 1 where
        ref maf: Mafioso,
        ref bg: Bodyguard,
        ref townie: Sheriff
    );

    maf.set_night_target(townie);
    bg.set_night_target(townie);
    
    kit::time::advance_phase(game); // Morning

    assert!(townie.get_messages().contains(&ChatMessage::BodyguardProtectedYou));

    assert!(townie.alive());
    assert!(!bg.alive());
    assert!(!maf.alive());
}

#[test]
fn sheriff_investigates() {
    kit::scenario!(game in Night 1 where
        ref sher: Sheriff,
        ref mafia: Mafioso,
        ref townie: Sheriff
    );
    sher.set_night_targets(vec![mafia]);
    
    kit::time::advance_phase(game); // Morning
    assert!(sher.get_messages().contains(&ChatMessage::SheriffResult { suspicious: true }));

    kit::time::skip_to_phase(game, PhaseType::Night);

    sher.set_night_targets(vec![townie]);
    
    kit::time::advance_phase(game); // Morning
    assert!(sher.get_messages().contains(&ChatMessage::SheriffResult { suspicious: false }));
}

/// A module for tests involving transporter interactions.
pub mod transporter_interactions {
    use super::*;

    /// Tests if transporter properly swaps, redirecting actions on their first target to their
    /// second. The vigilante will try to kill town1, which should end up killing town2.
    #[test]
    fn vigilante_kills_transportee() {
        kit::scenario!(game in Night 2 where
            ref trans: Transporter,
            ref vigi: Vigilante,
            ref town1: Sheriff,
            ref town2: Sheriff
        );
    
        trans.set_night_targets(vec![town1, town2]);
        vigi.set_night_target(town1);
    
        kit::time::advance_phase(game); // Morning
    
        // Regular transportation works (vigi)
        assert!(town1.alive());
        assert!(!town2.alive());
        
        kit::time::skip_days(game, 1); // Morning
    
        //Vigilante did suicide
        assert!(!vigi.alive());
    }

    /// Test if when the transporter swaps two people, visits to the second target also redirect to
    /// the first target. The escort will try to roleblock town2, which will end up roleblocking 
    /// town1.
    #[test]
    fn escort_roleblocks_reverse_transportee() {
        kit::scenario!(game in Night 1 where 
            ref trans: Transporter,
            ref escort: Escort,
            ref town1: Sheriff,
            ref town2: Sheriff
        );
    
        trans.set_night_targets(vec![town1, town2]);
        escort.set_night_target(town2);
    
        kit::time::advance_phase(game); // Morning
        
        // Reverse transportation works (esc)
        assert!(town1.was_roleblocked());
        assert!(!town2.was_roleblocked());
    }
    
    /// Test that the bodyguard protects the person their target was swapped with
    #[test]
    fn bodyguard_protects_transported_target() {
        kit::scenario!(game in Night 1 where
            ref trans: Transporter,
            ref maf: Mafioso,
            ref bg: Bodyguard,
            ref t1: Sheriff,
            ref t2: Sheriff
        );
    
        trans.set_night_targets(vec![t1, t2]);
        maf.set_night_target(t1);
        bg.set_night_target(t1);
        
        kit::time::advance_phase(game); // Morning
    
        assert!(t1.alive());
        assert!(t2.alive());
        assert!(trans.alive());
        assert!(!bg.alive());
        assert!(!maf.alive());
    
        assert!(t2.get_messages().contains(&ChatMessage::BodyguardProtectedYou));
    }
}