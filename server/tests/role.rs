mod kit;
pub(crate) use kit::{assert_contains, assert_not_contains};
use mafia_server::game::{chat::{MessageSender, ChatGroup}, role::{retributionist::Retributionist, jester::Jester}};
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
fn bodyguard_basic() {
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
fn sheriff_basic() {
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

/// Tests if transporter properly swaps, redirecting actions on their first target to their
/// second. The vigilante will try to kill town1, which should end up killing town2.
#[test]
fn transporter_basic_vigilante_escort() {
    kit::scenario!(game in Night 2 where
        trans: Transporter,
        vigi: Vigilante,
        escort: Escort,
        town1: Sheriff,
        town2: Sheriff
    );
    trans.set_night_targets(vec![town1, town2]);
    vigi.set_night_target(town1);
    escort.set_night_target(town2);

    game.skip_to(PhaseType::Morning, 3);
    assert!(town1.alive());
    assert!(!town2.alive());

    assert!(town1.was_roleblocked());
    assert!(!town2.was_roleblocked());
    
    game.skip_to(PhaseType::Morning, 4);
    assert!(!vigi.alive());
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

#[test]
fn retributionist_basic(){
    kit::scenario!(game where
        ret: Retributionist,
        sher1: Sheriff,
        sher2: Sheriff,
        mafioso: Mafioso,
        jester: Jester
    );
    sher1.die();
    sher2.die();

    game.next_phase();
    ret.set_night_targets(vec![sher1, mafioso]);
    game.next_phase();
    assert_eq!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(),
        ChatMessage::RetributionistBug{message: Box::new(
            ChatMessage::SheriffResult{ suspicious: true }
        )}
    );
    
    game.skip_to(PhaseType::Night, 2);
    ret.set_night_targets(vec![sher1, mafioso, jester]);
    game.next_phase();
    assert_ne!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(), 
        ChatMessage::RetributionistBug{message: Box::new(
            ChatMessage::SheriffResult{suspicious: true}
        )}
    );
    
    game.skip_to(PhaseType::Night, 3);
    ret.set_night_targets(vec![sher2, jester, mafioso]);
    game.next_phase();
    assert_eq!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(), 
        ChatMessage::RetributionistBug{message: Box::new(
            ChatMessage::SheriffResult{suspicious: false}
        )}
    );
}