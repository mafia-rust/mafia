mod kit;
use std::vec;

pub(crate) use kit::{assert_contains, assert_not_contains};

pub use mafia_server::game::{
    chat::{ChatMessage, MessageSender, ChatGroup}, 
    grave::*, 
    role_list::Faction,
    player::PlayerReference,
    tag::Tag,
    role::{
        Role,
        RoleState,

        jailor::Jailor,
        mayor::Mayor,
        transporter::Transporter,

        sheriff::Sheriff,
        lookout::Lookout,
        spy::{Spy, SpyBug},
        tracker::Tracker,
        seer::Seer,
        psychic::Psychic,

        doctor::Doctor,
        bodyguard::Bodyguard,
        crusader::Crusader,

        vigilante::Vigilante,
        veteran::Veteran,
        deputy::Deputy,

        escort::Escort,
        medium::Medium,
        retributionist::Retributionist,

        godfather::Godfather,
        mafioso::Mafioso,
        
        consort::Consort,
        blackmailer::Blackmailer,
        consigliere::Consigliere,
        witch::Witch,
        necromancer::Necromancer,

        janitor::Janitor,
        framer::Framer,

        jester::Jester,
        executioner::Executioner,
        doomsayer::{Doomsayer, DoomsayerGuess},

        death::Death,

        vampire::Vampire,
        amnesiac::Amnesiac
    }, 
    phase::{
        PhaseState, 
        PhaseType
    }
};
// Pub use so that submodules don't have to re-import everything.
pub use mafia_server::packet::ToServerPacket;

#[test]
fn medium_receives_dead_messages_from_jail() {
    kit::scenario!(game where
        medium: Medium,
        jailor: Jailor,
        townie: Sheriff,
        mafioso: Mafioso
    );
    game.next_phase();
    mafioso.set_night_target(townie);
    game.skip_to(PhaseType::Voting, 2);
    
    jailor.day_target(medium);

    game.next_phase();
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
        blackmailer: Blackmailer,
        esc: Escort,
        transp: Transporter,
        bugged: Sheriff,
        jester: Jester
    );
    spy.set_night_target(jester);
    transp.set_night_targets(vec![jester, bugged]);
    blackmailer.set_night_target(jester);
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

    assert!(townie.get_messages().contains(&ChatMessage::YouWereProtected));

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

    assert_not_contains!(t1.get_messages(), ChatMessage::YouWereProtected);
    assert_contains!(t2.get_messages(), ChatMessage::YouWereProtected);
    assert_contains!(bg.get_messages(), ChatMessage::TargetWasAttacked);
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

    game.next_phase();

    mafioso.set_night_target(sher1);
    game.skip_to(PhaseType::Night, 2);
    mafioso.set_night_target(sher2);
    game.skip_to(PhaseType::Night, 3);

    assert!(!sher1.alive());
    assert!(!sher2.alive());

    assert!(ret.set_night_targets(vec![sher1, mafioso]));
    game.next_phase();
    assert_eq!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(),
        ChatMessage::RetributionistMessage{message: Box::new(
            ChatMessage::SheriffResult{ suspicious: true }
        )}
    );
    
    game.skip_to(PhaseType::Night, 4);
    assert!(!ret.set_night_targets(vec![sher1, mafioso, jester]));
    game.next_phase();
    assert_ne!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(), 
        ChatMessage::RetributionistMessage{message: Box::new(
            ChatMessage::SheriffResult{suspicious: true}
        )}
    );
    
    game.skip_to(PhaseType::Night, 5);
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
        jester: Jester,
        vigilante: Vigilante
    );
    
    game.next_phase();
    mafioso.set_night_target(sher);
    game.skip_to(PhaseType::Night, 2);
    vigilante.set_night_target(consigliere);
    game.skip_to(PhaseType::Night, 3);



    assert!(ret.set_night_targets(vec![sher, mafioso]));
    game.next_phase();
    assert_eq!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(),
        ChatMessage::NecromancerMessage{message: Box::new(
            ChatMessage::SheriffResult{ suspicious: true }
        )}
    );
    
    game.skip_to(PhaseType::Night, 4);
    assert!(!ret.set_night_targets(vec![sher, mafioso, jester]));
    game.next_phase();
    assert_ne!(
        *ret.get_messages().get(ret.get_messages().len()-2).unwrap(), 
        ChatMessage::NecromancerMessage{message: Box::new(
            ChatMessage::SheriffResult{suspicious: true}
        )}
    );
    
    game.skip_to(PhaseType::Night, 5);
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
fn crusader_does_not_kill_framed_player(){
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
fn veteran_does_not_kill_framed_player(){
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

#[test]
fn transporter_cant_transport_dead() {
    kit::scenario!(game in Night 1 where
        mafioso: Mafioso,
        _vet: Veteran,
        _necro: Necromancer,
        _seer: Seer,
        townie: Sheriff,
        thomas: Jailor,
        trans: Transporter
    );

    mafioso.set_night_target(thomas);

    game.next_phase();

    assert!(!thomas.alive());

    game.skip_to(PhaseType::Night, 2);

    assert!(trans.set_night_target(townie));
    assert!(!trans.set_night_targets(vec![townie, thomas]), "Transporter targeted dead player");

    game.next_phase();

    assert_not_contains!(thomas.get_messages(), ChatMessage::Transported);
    assert_not_contains!(townie.get_messages(), ChatMessage::Transported);
}

#[test]
fn double_transport() {
    kit::scenario!(game in Night 1 where
        mafioso: Mafioso,
 
        townie_a: Sheriff,
        townie_b: Jailor,

        trans_a: Transporter,
        trans_b: Transporter
    );
    
    assert!(mafioso.set_night_target(townie_a));

    assert!(trans_a.set_night_targets(vec![townie_a, townie_b]));
    assert!(trans_b.set_night_targets(vec![townie_b, townie_a]));

    game.next_phase();
    assert!(!townie_a.alive());
    assert!(townie_b.alive());
}


#[test]
fn double_transport_single_player() {
    kit::scenario!(game in Night 1 where
        mafioso: Mafioso,
 
        townie_a: Sheriff,
        townie_b: Jailor,
        townie_c: Vigilante,

        trans_a: Transporter,
        trans_b: Transporter
    );
    
    assert!(mafioso.set_night_target(townie_a));

    assert!(trans_a.set_night_targets(vec![townie_a, townie_b]));
    assert!(trans_b.set_night_targets(vec![townie_a, townie_c]));


    game.next_phase();
    assert!(townie_a.alive());
    assert!(!townie_b.alive());
    assert!(townie_c.alive());
}

#[test]
fn double_transport_three_players() {
    kit::scenario!(game in Night 1 where
        mafioso: Mafioso,
 
        townie_a: Sheriff,
        townie_b: Jailor,
        townie_c: Vigilante,

        trans_a: Transporter,
        trans_b: Transporter,
        trans_c: Transporter
    );
    
    assert!(mafioso.set_night_target(townie_a));

    assert!(trans_a.set_night_targets(vec![townie_a, townie_b]));
    assert!(trans_b.set_night_targets(vec![townie_a, townie_c]));
    assert!(trans_c.set_night_targets(vec![townie_b, townie_c]));


    game.next_phase();
    assert!(townie_a.alive());
    assert!(townie_b.alive());
    assert!(!townie_c.alive());
}


#[test]
fn grave_contains_multiple_killers() {
    kit::scenario!(game in Night 2 where
        mafioso: Mafioso,
        vigilante: Vigilante,
        townie: Sheriff
    );

    assert!(mafioso.set_night_target(townie));
    assert!(vigilante.set_night_target(townie));
    game.next_phase();
    assert_eq!(
        *game.graves.first().unwrap(), 
        Grave {
            player: townie.index(),
        
            role: GraveRole::Role(Role::Sheriff),
            death_cause: GraveDeathCause::Killers(vec![GraveKiller::Faction(Faction::Mafia), GraveKiller::Role(Role::Vigilante)]),
            will: "".to_string(),
            death_notes: vec![],
        
            died_phase: GravePhase::Night,
            day_number: 2,
    });
}

#[test]
fn grave_contains_multiple_killers_roles() {
    kit::scenario!(game in Night 2 where
        townie_b: Doctor,
        _townie_a: Doctor,
        mafioso: Mafioso,
        vigilante: Vigilante,
        doom: Doomsayer
    );

    assert!(mafioso.set_night_target(townie_b));
    assert!(vigilante.set_night_target(townie_b));
    doom.set_role_state(RoleState::Doomsayer(
        Doomsayer { guesses: [
            (PlayerReference::new(&game, 0).expect("that player doesnt exist"), DoomsayerGuess::Doctor),
            (PlayerReference::new(&game, 1).expect("that player doesnt exist"), DoomsayerGuess::Doctor),
            (PlayerReference::new(&game, 2).expect("that player doesnt exist"), DoomsayerGuess::Mafia)
        ],
        won: doom.get_won_game()
    }));


    game.next_phase();
    assert_eq!(
        *game.graves.first().unwrap(), 
        Grave {
            player: townie_b.index(),
        
            role: GraveRole::Role(Role::Doctor),
            death_cause: GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Doomsayer), GraveKiller::Faction(Faction::Mafia), GraveKiller::Role(Role::Vigilante)]),
            will: "".to_string(),
            death_notes: vec![],
        
            died_phase: GravePhase::Night,
            day_number: 2,
    });
}

#[test]
fn godfathers_backup_tag_works() {
    kit::scenario!(game in Night 2 where
        godfather: Godfather,
        blackmailer: Blackmailer,
        consort: Consort,
        _vigi: Vigilante
    );

    assert!(godfather.day_target(blackmailer));
    assert!(blackmailer.get_player_tags().get(&blackmailer.player_ref()).expect("blackmailer doesnt have tag").contains(&Tag::GodfatherBackup));
    
    assert!(godfather.day_target(blackmailer));
    assert!(blackmailer.get_player_tags().get(&blackmailer.player_ref()).is_none());

    assert!(godfather.day_target(blackmailer));
    assert!(blackmailer.get_player_tags().get(&blackmailer.player_ref()).expect("blackmailer doesnt have tag").contains(&Tag::GodfatherBackup));
    
    assert!(godfather.day_target(consort));
    assert!(blackmailer.get_player_tags().get(&consort.player_ref()).expect("consort doesnt have tag").contains(&Tag::GodfatherBackup));
    assert!(blackmailer.get_player_tags().get(&blackmailer.player_ref()).is_none());
}