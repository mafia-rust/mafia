mod kit;
use std::vec;

pub(crate) use kit::{assert_contains, assert_not_contains};
use mafia_server::game::{chat::{MessageSender, ChatGroup}, role::{retributionist::Retributionist, jester::Jester, crusader::Crusader, framer::Framer, veteran::Veteran, executioner::Executioner, spy::{Spy, SpyBug}, blackmailer::Blackmailer, vampire::Vampire, doctor::Doctor, Role, janitor::Janitor, consigliere::Consigliere, necromancer::Necromancer, witch::Witch, seer::Seer}};
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

#[test]
fn seer_basic() {
    kit::scenario!(game in Night 1 where
        seer: Seer,
        mafia1: Mafioso,
        mafia2: Consigliere,
        townie1: Sheriff,
        townie2: Vigilante,
        jester: Jester
    );
    seer.set_night_targets(vec![mafia1, townie1]);
    
    game.skip_to(PhaseType::Morning, 2);
    assert_eq!(
        *seer.get_messages().get(seer.get_messages().len()-2).unwrap(),
        ChatMessage::SeerResult { enemies: true }
    );

    game.skip_to(PhaseType::Night, 2);
    seer.set_night_targets(vec![mafia1, mafia2]);
    
    game.skip_to(PhaseType::Morning, 3);
    assert_eq!(
        *seer.get_messages().get(seer.get_messages().len()-2).unwrap(),
        ChatMessage::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 3);
    seer.set_night_targets(vec![jester, mafia2]);
    
    game.skip_to(PhaseType::Morning, 4);
    assert_eq!(
        *seer.get_messages().get(seer.get_messages().len()-2).unwrap(),
        ChatMessage::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 4);
    seer.set_night_targets(vec![townie2, jester]);
    
    game.skip_to(PhaseType::Morning, 5);
    assert_eq!(
        *seer.get_messages().get(seer.get_messages().len()-2).unwrap(),
        ChatMessage::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 5);
    seer.set_night_targets(vec![townie2, townie1]);
    
    game.skip_to(PhaseType::Morning, 6);
    assert_eq!(
        *seer.get_messages().get(seer.get_messages().len()-2).unwrap(),
        ChatMessage::SeerResult { enemies: false }
    );
}

#[test]
fn spy_basic_transported() {
    kit::scenario!(game in Night 1 where
        spy: Spy,
        _mafioso: Mafioso,
        bmer: Blackmailer,
        esc: Escort,
        transp: Transporter,
        bugged: Sheriff,
        jester: Jester
    );
    spy.set_night_target(jester);
    transp.set_night_targets(vec![jester, bugged]);
    bmer.set_night_target(jester);
    esc.set_night_target(jester);

    game.next_phase();

    assert_contains!(spy.get_messages(), ChatMessage::SpyBug { bug: SpyBug::Silenced });
    assert_contains!(spy.get_messages(), ChatMessage::SpyBug { bug: SpyBug::Roleblocked });
    assert_contains!(spy.get_messages(), ChatMessage::SpyBug { bug: SpyBug::Transported });

    
    assert_contains!(spy.get_messages(), ChatMessage::SpyMafiaVisit { players: vec![bugged.index()] });
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

    assert!(townie.get_messages().contains(&ChatMessage::ProtectedYou));

    assert!(townie.alive());
    assert!(!bg.alive());
    assert!(!maf.alive());
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

#[test]
fn transporter_basic_seer_sheriff_framer() {
    kit::scenario!(game in Night 1 where
        trans: Transporter,
        seer: Seer,
        _mafioso: Mafioso,
        framer: Framer,
        town1: Sheriff,
        town2: Sheriff
    );
    assert!(trans.set_night_targets(vec![town1, town2]));
    assert!(framer.set_night_targets(vec![town1, seer]));
    assert!(seer.set_night_targets(vec![town1, town2]));
    assert!(town1.set_night_targets(vec![town2]));
    assert!(town2.set_night_targets(vec![town1]));

    game.skip_to(PhaseType::Morning, 2);
    assert_eq!(
        *seer.get_messages().get(seer.get_messages().len()-2).unwrap(),
        ChatMessage::SeerResult { enemies: true }
    );
    assert_eq!(
        *town1.get_messages().get(town1.get_messages().len()-2).unwrap(),
        ChatMessage::SheriffResult { suspicious: false }
    );
    assert_eq!(
        *town2.get_messages().get(town2.get_messages().len()-2).unwrap(),
        ChatMessage::SheriffResult { suspicious: true }
    );
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

    assert_not_contains!(t1.get_messages(), ChatMessage::ProtectedYou);
    assert_contains!(t2.get_messages(), ChatMessage::ProtectedYou);
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
    assert!(ret.set_night_targets(vec![sher1, mafioso]));
    game.next_phase();
    assert_eq!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(),
        ChatMessage::RetributionistMessage{message: Box::new(
            ChatMessage::SheriffResult{ suspicious: true }
        )}
    );
    
    game.skip_to(PhaseType::Night, 2);
    assert!(!ret.set_night_targets(vec![sher1, mafioso, jester]));
    game.next_phase();
    assert_ne!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(), 
        ChatMessage::RetributionistMessage{message: Box::new(
            ChatMessage::SheriffResult{suspicious: true}
        )}
    );
    
    game.skip_to(PhaseType::Night, 3);
    assert!(ret.set_night_targets(vec![sher2, jester, mafioso]));
    game.next_phase();
    assert_eq!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(), 
        ChatMessage::RetributionistMessage{message: Box::new(
            ChatMessage::SheriffResult{suspicious: false}
        )}
    );
}

#[test]
fn necromancer_basic(){
    kit::scenario!(game where
        ret: Necromancer,
        sher: Sheriff,
        consigliere: Consigliere,
        mafioso: Mafioso,
        jester: Jester
    );
    sher.die();
    consigliere.die();

    game.next_phase();
    assert!(ret.set_night_targets(vec![sher, mafioso]));
    game.next_phase();
    assert_eq!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(),
        ChatMessage::NecromancerMessage{message: Box::new(
            ChatMessage::SheriffResult{ suspicious: true }
        )}
    );
    
    game.skip_to(PhaseType::Night, 2);
    assert!(!ret.set_night_targets(vec![sher, mafioso, jester]));
    game.next_phase();
    assert_ne!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(), 
        ChatMessage::NecromancerMessage{message: Box::new(
            ChatMessage::SheriffResult{suspicious: true}
        )}
    );
    
    game.skip_to(PhaseType::Night, 3);
    assert!(ret.set_night_targets(vec![consigliere, jester, mafioso]));
    game.next_phase();
    assert_eq!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(), 
        ChatMessage::NecromancerMessage{message: Box::new(
            ChatMessage::ConsigliereResult { role: Role::Jester, visited_by: vec![], visited: vec![] }
        )}
    );
}

#[test]
fn witch_basic(){
    kit::scenario!(game where
        witch: Witch,
        sher: Sheriff,
        consigliere: Consigliere,
        mafioso: Mafioso,
        jester: Jester
    );

    game.next_phase();
    assert!(witch.set_night_targets(vec![sher, mafioso]));
    game.next_phase();
    assert_eq!(
        *witch.get_messages().get(witch.get_messages().len()-2).unwrap(),
        ChatMessage::WitchMessage{message: Box::new(
            ChatMessage::SheriffResult{ suspicious: true }
        )}
    );
    
    game.skip_to(PhaseType::Night, 2);
    assert!(witch.set_night_targets(vec![sher, mafioso, jester]));
    game.next_phase();
    assert_eq!(
        *witch.get_messages().get(witch.get_messages().len()-2).unwrap(), 
        ChatMessage::WitchMessage{message: Box::new(
            ChatMessage::SheriffResult{suspicious: true}
        )}
    );
    
    game.skip_to(PhaseType::Night, 3);
    assert!(!witch.set_night_targets(vec![consigliere, jester, mafioso]));
    game.next_phase();
    assert_ne!(
        *witch.get_messages().get(witch.get_messages().len()-2).unwrap(), 
        ChatMessage::WitchMessage{message: Box::new(
            ChatMessage::ConsigliereResult { role: Role::Jester, visited_by: vec![], visited: vec![] }
        )}
    );
}

#[test]
fn crusader_basic(){
    kit::scenario!(game in Night 1 where
        crus: Crusader,
        protected: Jester,
        townie1: Sheriff,
        townie2: Sheriff,
        mafioso: Mafioso
    );

    crus.set_night_targets(vec![protected]);
    townie1.set_night_targets(vec![protected]);
    townie2.set_night_targets(vec![protected]);
    mafioso.set_night_targets(vec![protected]);

    game.skip_to(PhaseType::Night, 2);

    assert!(crus.alive());
    assert!(protected.alive());
    assert!(townie1.alive());
    assert!(townie2.alive());
    assert!(!mafioso.alive());

    crus.set_night_targets(vec![protected]);
    townie1.set_night_targets(vec![protected]);
    townie2.set_night_targets(vec![protected]);

    game.next_phase();
    
    assert!(crus.alive());
    assert!(protected.alive());
    assert!(townie1.alive() || townie2.alive());
    assert!(!(townie1.alive() && townie2.alive()));
}

#[test]
fn crusader_doesnt_kill_framed_player(){
    kit::scenario!(game in Night 1 where
        crus: Crusader,
        protected: Jester,
        townie: Sheriff,
        framer: Framer,
        mafioso: Mafioso
    );

    assert!(crus.set_night_targets(vec![protected]));
    assert!(framer.set_night_targets(vec![townie, protected]));

    game.next_phase();

    assert!(crus.alive());
    assert!(protected.alive());
    assert!(framer.alive());
    assert!(mafioso.alive());
    assert!(townie.alive());
}

#[test]
fn veteran_doesnt_kill_framed_player(){
    kit::scenario!(game in Night 1 where
        vet: Veteran,
        townie: Sheriff,
        framer: Framer,
        mafioso: Mafioso
    );

    assert!(vet.set_night_targets(vec![vet]));
    assert!(framer.set_night_targets(vec![townie, vet]));

    game.next_phase();

    assert!(vet.alive());
    assert!(framer.alive());
    assert!(mafioso.alive());
    assert!(townie.alive());
}

#[test]
fn executioner_turns_into_jester(){
    kit::scenario!(game in Night 1 where
        target: Sheriff,
        mafioso: Mafioso,
        exe: Executioner
    );

    assert!(mafioso.set_night_targets(vec![target]));

    game.skip_to(PhaseType::Voting, 2);

    assert!(!target.alive());
    assert!(exe.alive());
    assert!(mafioso.alive());
    let RoleState::Jester(_) = exe.role_state() else {panic!()};
}
#[test]
fn executioner_instantly_turns_into_jester(){
    kit::scenario!(_game where
        exe: Executioner
    );
    let RoleState::Jester(_) = exe.role_state() else {panic!()};
}

#[test]
fn vampire_cant_convert_protected(){
    kit::scenario!(game where
        vamp: Vampire,
        doc: Doctor,
        sher: Sheriff
    );

    game.next_phase();

    assert!(doc.set_night_targets(vec![sher]));
    assert!(vamp.set_night_targets(vec![sher]));

    game.skip_to(PhaseType::Night, 2);

    assert!(sher.role_state().role() != Role::Vampire);
    
    assert!(vamp.set_night_targets(vec![sher]));

    game.next_phase();

    assert!(sher.role_state().role() == Role::Vampire);
}

#[test]
fn vampire_cant_convert_twice_in_a_row(){
    kit::scenario!(game where
        vamp: Vampire,
        c1: Sheriff,
        c2: Sheriff
    );

    //first convert
    game.next_phase();
    assert!(vamp.set_night_targets(vec![c1]));

    //convert worked
    game.next_phase();
    assert!(c1.role_state().role() == Role::Vampire);
    assert!(c2.role_state().role() != Role::Vampire);

    //second convert should fail
    game.skip_to(PhaseType::Night, 2);

    assert!(!vamp.set_night_targets(vec![c2]));
    assert!(!c1.set_night_targets(vec![c2]));

    //check convert failed
    game.next_phase();
    
    assert!(c1.role_state().role() == Role::Vampire);
    assert!(c2.role_state().role() != Role::Vampire);

    //second attempt at second convert
    game.skip_to(PhaseType::Night, 3);

    assert!(!vamp.set_night_targets(vec![c2]));
    assert!(c1.set_night_targets(vec![c2]));

    //final convert should work
    game.next_phase();
    
    assert!(c1.role_state().role() == Role::Vampire);
    assert!(c2.role_state().role() == Role::Vampire);

}

#[test]
fn can_type_in_jail() {
    kit::scenario!(game where
        jailor: Jailor,
        sheriff: Sheriff
    );

    jailor.day_target(sheriff);
    game.next_phase();

    sheriff.send_message("Hello!");
    
    assert_contains!(jailor.get_messages(), 
        ChatMessage::Normal { 
            message_sender: MessageSender::Player { player: sheriff.index() }, 
            text: "Hello!".to_string(), 
            chat_group: ChatGroup::Jail
        }
    );
    
    assert_contains!(sheriff.get_messages(), 
        ChatMessage::Normal { 
            message_sender: MessageSender::Player { player: sheriff.index() }, 
            text: "Hello!".to_string(), 
            chat_group: ChatGroup::Jail
        }
    );
}

#[test]
fn mafioso_cant_kill_mafia() {
    kit::scenario!(game in Night 1 where
        mafioso: Mafioso,
        janitor: Janitor
    );

    mafioso.set_night_target(janitor);

    game.next_phase();

    assert!(janitor.alive());
}
