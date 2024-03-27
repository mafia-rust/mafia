mod kit;
use std::vec;

pub(crate) use kit::{assert_contains, assert_not_contains};

use mafia_server::game::role::{martyr::Martyr, reveler::Reveler};
pub use mafia_server::game::{
    chat::{ChatMessageVariant, MessageSender, ChatGroup}, 
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

        apostle::Apostle,
        amnesiac::Amnesiac
    }, 
    phase::{
        PhaseState, 
        PhaseType
    }
};
// Pub use so that submodules don't have to reimport everything.
pub use mafia_server::packet::ToServerPacket;

#[test]
fn medium_receives_dead_messages_from_jail() {
    kit::scenario!(game in Night 1 where
        medium: Medium,
        jailor: Jailor,
        townie: Sheriff,
        mafioso: Mafioso
    );
    mafioso.set_night_target(townie);
    game.skip_to(PhaseType::Nomination, 2);
    
    jailor.day_target(medium);

    game.skip_to(PhaseType::Night, 2);
    let dead_message = "Hello medium!! Are you there!?";
    townie.send_message(dead_message);

    assert_contains!(medium.get_messages(), 
        ChatMessageVariant::Normal { 
            text: dead_message.to_string(),
            message_sender: MessageSender::Player { player: townie.index() }
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
    
    game.skip_to(PhaseType::Obituary, 2);
    assert_contains!(sher.get_messages(), ChatMessageVariant::SheriffResult { suspicious: true });

    game.skip_to(PhaseType::Night, 2);
    sher.set_night_targets(vec![townie]);
    
    game.skip_to(PhaseType::Obituary, 3);
    assert_contains!(sher.get_messages(), ChatMessageVariant::SheriffResult { suspicious: false });
}

#[test]
fn sheriff_godfather() {
    kit::scenario!(game in Night 1 where
        sher: Sheriff,
        mafia: Godfather
    );
    sher.set_night_targets(vec![mafia]);
    
    game.skip_to(PhaseType::Obituary, 2);
    assert_contains!(sher.get_messages(), ChatMessageVariant::SheriffResult { suspicious: false });
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
    
    game.skip_to(PhaseType::Obituary, 2);
    assert_contains!(
        seer.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1}),
        ChatMessageVariant::SeerResult { enemies: true }
    );

    game.skip_to(PhaseType::Night, 2);
    seer.set_night_targets(vec![mafia1, mafia2]);
    
    game.skip_to(PhaseType::Obituary, 3);
    assert_contains!(
        seer.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 2}),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 3);
    seer.set_night_targets(vec![jester, mafia2]);
    
    game.skip_to(PhaseType::Obituary, 4);
    assert_contains!(
        seer.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 3}),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 4);
    seer.set_night_targets(vec![townie2, jester]);
    
    game.skip_to(PhaseType::Obituary, 5);
    assert_contains!(
        seer.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 4}),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 5);
    seer.set_night_targets(vec![townie2, townie1]);
    
    game.skip_to(PhaseType::Obituary, 6);
    assert_contains!(
        seer.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 5}),
        ChatMessageVariant::SeerResult { enemies: false }
    );
}

#[test]
fn psychic_auras(){
    for _ in 0..100 {
        kit::scenario!(game in Night 1 where
            psy: Psychic,
            god: Godfather,
            maf: Framer,
            town1: Sheriff,
            town2: Vigilante
        );
    
        game.next_phase();
        let messages = psy.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1});
        let messages: Vec<_> = 
            messages.into_iter()
            .filter(|msg|match msg {
                ChatMessageVariant::PsychicEvil { players } => {
                    players.contains(&maf.index()) &&
                    !players.contains(&god.index()) &&
                    !players.contains(&psy.index())
                }
                _ => false
            }).collect();

        if messages.len() != 1 {
            panic!("{:?}", messages);
        }

        game.skip_to(PhaseType::Night, 2);
        maf.set_night_target(town1);
        game.next_phase();
        let messages = psy.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 2});
        let messages: Vec<_> = 
            messages.into_iter()
            .filter(|msg|match msg {
                ChatMessageVariant::PsychicGood { players } => {
                    players.contains(&town2.index()) &&
                    !players.contains(&town1.index()) &&
                    !players.contains(&psy.index())
                }
                _ => false
            }).collect();

        if messages.len() != 1 {
            panic!("{:?}", messages);
        }
    }
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
        jester: Jester,
        witch: Witch
    );
    spy.set_night_target(jester);
    transp.set_night_targets(vec![jester, bugged]);
    blackmailer.set_night_target(jester);
    esc.set_night_target(jester);
    witch.set_night_targets(vec![jester, esc]);

    game.next_phase();

    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyBug { bug: SpyBug::Silenced });
    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyBug { bug: SpyBug::Roleblocked });
    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyBug { bug: SpyBug::Transported });
    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyBug { bug: SpyBug::Possessed });

    
    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyMafiaVisit { players: vec![bugged.index(), bugged.index()] });
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

    game.skip_to(PhaseType::Obituary, 2);

    assert!(townie.get_messages().contains(&ChatMessageVariant::YouWereProtected));

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

    game.skip_to(PhaseType::Obituary, 3);
    assert!(town1.alive());
    assert!(!town2.alive());

    assert!(town1.was_roleblocked());
    assert!(!town2.was_roleblocked());
    
    game.skip_to(PhaseType::Obituary, 4);
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

    game.skip_to(PhaseType::Obituary, 2);
    assert_contains!(
        seer.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1 }),
        ChatMessageVariant::SeerResult { enemies: true }
    );
    assert_contains!(
        town1.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1 }),
        ChatMessageVariant::SheriffResult { suspicious: false }
    );
    assert_contains!(
        town2.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1 }),
        ChatMessageVariant::SheriffResult { suspicious: true }
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
    
    game.skip_to(PhaseType::Obituary, 2);
    assert!(t1.alive());
    assert!(t2.alive());
    assert!(trans.alive());
    assert!(!bg.alive());
    assert!(!maf.alive());

    assert_not_contains!(t1.get_messages(), ChatMessageVariant::YouWereProtected);
    assert_contains!(t2.get_messages(), ChatMessageVariant::YouWereProtected);
    assert_contains!(bg.get_messages(), ChatMessageVariant::TargetWasAttacked);
}

#[test]
fn mayor_reveals_after_they_vote(){
    kit::scenario!(game where
        mayor: Mayor,
        _townie: Sheriff,
        mafioso: Mafioso
    );

    game.skip_to(PhaseType::Nomination, 2);
    mayor.vote_for_player(Some(mafioso));
    mayor.day_target(mayor);
    assert_eq!(game.current_phase().phase(), PhaseType::Testimony);
}


#[test]
fn retributionist_basic(){
    kit::scenario!(game in Night 1 where
        ret: Retributionist,
        sher1: Sheriff,
        sher2: Sheriff,
        mafioso: Mafioso
    );

    mafioso.set_night_target(sher1);
    game.skip_to(PhaseType::Night, 2);
    mafioso.set_night_target(sher2);
    game.skip_to(PhaseType::Night, 3);

    assert!(!sher1.alive());
    assert!(!sher2.alive());

    assert!(ret.set_night_targets(vec![sher1, mafioso]));
    game.next_phase();
    assert_contains!(
        ret.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 3 }),
        ChatMessageVariant::TargetsMessage{message: Box::new(
            ChatMessageVariant::SheriffResult{ suspicious: true }
        )}
    );
}

#[test]
fn necromancer_basic(){
    kit::scenario!(game in Night 1 where
        ret: Necromancer,
        sher: Sheriff,
        consigliere: Consigliere,
        mafioso: Mafioso,
        vigilante: Vigilante
    );
    
    mafioso.set_night_target(sher);
    game.skip_to(PhaseType::Night, 2);
    vigilante.set_night_target(consigliere);
    game.skip_to(PhaseType::Night, 3);



    assert!(ret.set_night_targets(vec![sher, mafioso]));
    game.next_phase();
    assert_contains!(
        ret.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 3 }),
        ChatMessageVariant::TargetsMessage{message: Box::new(
            ChatMessageVariant::SheriffResult{ suspicious: true }
        )}
    );
}

#[test]
fn witch_basic(){
    kit::scenario!(game in Night 1 where
        witch: Witch,
        sher: Sheriff,
        consigliere: Consigliere,
        mafioso: Mafioso,
        seer: Seer
    );

    assert!(witch.set_night_targets(vec![sher, mafioso]));
    game.next_phase();
    assert_contains!(witch.get_messages(), ChatMessageVariant::TargetsMessage{message: Box::new(
        ChatMessageVariant::SheriffResult{ suspicious: true }
    )});
    
    game.skip_to(PhaseType::Night, 2);
    assert!(seer.set_night_targets(vec![sher, consigliere]));
    assert!(witch.set_night_targets(vec![seer, mafioso]));
    game.next_phase();
    assert_contains!(
        witch.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 2 }),
        ChatMessageVariant::TargetsMessage{message: Box::new(
            ChatMessageVariant::SeerResult { enemies: false }
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

    game.skip_to(PhaseType::Nomination, 2);

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
fn can_type_in_jail() {
    kit::scenario!(game in Dusk 1 where
        jailor: Jailor,
        sheriff: Sheriff
    );

    jailor.day_target(sheriff);
    game.next_phase();

    sheriff.send_message("Hello!");
    
    assert_contains!(jailor.get_messages(), 
        ChatMessageVariant::Normal { 
            message_sender: MessageSender::Player { player: sheriff.index() }, 
            text: "Hello!".to_string()
        }
    );
    
    assert_contains!(sheriff.get_messages(), 
        ChatMessageVariant::Normal { 
            message_sender: MessageSender::Player { player: sheriff.index() }, 
            text: "Hello!".to_string()
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

    assert_not_contains!(thomas.get_messages(), ChatMessageVariant::Transported);
    assert_not_contains!(townie.get_messages(), ChatMessageVariant::Transported);
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
            player: townie.player_ref(),
        
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
            player: townie_b.player_ref(),
        
            role: GraveRole::Role(Role::Doctor),
            death_cause: GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Doomsayer), GraveKiller::Faction(Faction::Mafia), GraveKiller::Role(Role::Vigilante)]),
            will: "".to_string(),
            death_notes: vec![],
        
            died_phase: GravePhase::Night,
            day_number: 2,
    });
}
#[test]
fn vigilante_cant_select_night_one() {
    kit::scenario!(game in Night 1 where
        townie_b: Doctor,
        _godfather: Godfather,
        vigilante_suicide: Vigilante

    );
    assert!(!vigilante_suicide.set_night_target(townie_b));
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

#[test]
fn seer_cant_see_godfather() {
    kit::scenario!(game in Night 1 where
        seer: Seer,
        godfather: Godfather,
        mafioso: Mafioso,
        townie: Sheriff
    );

    assert!(seer.set_night_targets(vec![godfather, mafioso]));
    game.next_phase();
    assert_contains!(
        seer.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1 }
        ),
        ChatMessageVariant::SeerResult { enemies: false }
    );
    game.skip_to(PhaseType::Night, 2);

    assert!(seer.set_night_targets(vec![godfather, townie]));
    game.next_phase();
    assert_contains!(
        seer.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 2 }
        ),
        ChatMessageVariant::SeerResult { enemies: false }
    );
}

#[test]
fn reveler_protect_still_kill() {
    kit::scenario!(game in Night 1 where
        rev: Reveler,
        godfather: Godfather,
        jan: Janitor,
        townie_a: Sheriff,
        townie_b: Sheriff
    );

    assert!(rev.set_night_targets(vec![townie_a]));
    assert!(godfather.set_night_targets(vec![townie_a]));
    assert!(godfather.day_target(jan));
    assert!(jan.set_night_targets(vec![townie_b]));

    game.next_phase();
    assert_not_contains!(
        townie_a.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 1}
        ),
        ChatMessageVariant::RoleBlocked{immune: false}
    );
    assert_contains!(
        godfather.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 1}
        ),
        ChatMessageVariant::RoleBlocked{immune: false}
    );
    assert_not_contains!(
        jan.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 1}
        ),
        ChatMessageVariant::RoleBlocked{immune: false}
    );

    assert!(!townie_b.alive());
    assert!(townie_a.alive());
}

#[test]
fn cult_wait_for_two_deaths() {
    kit::scenario!(game in Night 1 where
        drac: Apostle,
        a: Sheriff,
        b: Sheriff,
        c: Sheriff,
        d: Sheriff,
        e: Sheriff,
        f: Sheriff,
        g: Sheriff,
        h: Sheriff,
        i: Sheriff,
        j: Sheriff
    );

    //apostle kills
    assert!(drac.set_night_targets(vec![a]));
    game.next_phase();
    assert!(!a.alive());
    assert!(a.role_state().role().faction() != Faction::Cult);

    //apostle converts
    game.skip_to(PhaseType::Night, 2);
    assert!(drac.set_night_targets(vec![b]));
    game.next_phase();
    assert!(b.alive());
    assert!(b.role_state().role().faction() == Faction::Cult);

    //zealot kills, apostle waits
    game.skip_to(PhaseType::Night, 3);
    assert!(!drac.set_night_targets(vec![d]));
    assert!(b.set_night_targets(vec![c]));
    game.next_phase();
    assert!(!c.alive());
    assert!(d.alive());
    assert!(d.role_state().role().faction() != Faction::Cult);

    //zealot kills, apostle waits
    game.skip_to(PhaseType::Night, 4);
    assert!(!drac.set_night_targets(vec![d]));
    assert!(b.set_night_targets(vec![e]));
    game.next_phase();
    assert!(!e.alive());
    assert!(d.alive());
    assert!(d.role_state().role().faction() != Faction::Cult);

    //zealot kills, apostle converts
    game.skip_to(PhaseType::Night, 5);
    assert!(drac.set_night_targets(vec![f]));
    assert!(b.set_night_targets(vec![g]));
    game.next_phase();
    assert!(f.alive());
    assert!(f.role_state().role().faction() == Faction::Cult);
    assert!(!g.alive());

    //zealot kills, apostle waits
    game.skip_to(PhaseType::Night, 6);
    assert!(!drac.set_night_targets(vec![h]));
    assert!(f.set_night_targets(vec![i]));
    game.next_phase();
    assert!(!i.alive());
    assert!(h.alive());
    assert!(h.role_state().role().faction() != Faction::Cult);

    //zealot kills, apostle converts, same person
    game.skip_to(PhaseType::Night, 7);
    assert!(drac.set_night_targets(vec![j]));
    assert!(f.set_night_targets(vec![j]));
    game.next_phase();
    assert!(!j.alive());
    assert!(j.role_state().role().faction() == Faction::Cult);



}

#[test]
fn bodyguard_gets_single_target_jailed_message() {
    kit::scenario!(game in Dusk 1 where
        bg: Bodyguard,
        jailor: Jailor,
        _maf: Mafioso,
        townie: Sheriff
    );

    jailor.day_target(townie);

    game.next_phase();

    bg.set_night_target(townie);

    game.next_phase();

    assert_eq!(
        bg.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { 
                phase: PhaseState::Night, day_number: 1
            }
        ),
        vec![
            ChatMessageVariant::TargetJailed,
            /* They should not get a second TargetJailed message */
            ChatMessageVariant::PhaseChange { 
                phase: PhaseState::Obituary, day_number: 2 
            }
        ]
    );
}

#[test]
fn martyr_suicide_ends_game() {
    kit::scenario!(game in Night 1 where
        martyr: Martyr,
        player1: Mafioso,
        player2: Sheriff,
        player3: Mafioso,
        player4: Sheriff
    );

    assert_contains!(
        player1.get_messages(),
        ChatMessageVariant::MartyrRevealed { martyr: martyr.index() }
    );

    martyr.set_night_target(martyr);

    game.next_phase();

    assert!(!martyr.alive());
    assert!(martyr.role_state().clone().get_won_game(&game, martyr.player_ref()));
    assert!(!player1.alive());
    assert!(!player2.alive());
    assert!(!player3.alive());
    assert!(!player4.alive());

    assert_contains!(
        player1.get_messages(),
        ChatMessageVariant::MartyrWon
    );

    assert!(game.game_is_over());
}

#[test]
fn martyr_roleblocked() {
    kit::scenario!(game in Night 1 where
        martyr: Martyr,
        player1: Mafioso,
        player2: Sheriff,
        consort: Consort,
        player4: Sheriff
    );

    assert_contains!(
        player1.get_messages(),
        ChatMessageVariant::MartyrRevealed { martyr: martyr.index() }
    );

    martyr.set_night_target(martyr);
    consort.set_night_target(martyr);

    game.next_phase();

    assert!(martyr.alive());
    assert!(!martyr.role_state().clone().get_won_game(&game, martyr.player_ref()));
    assert!(player1.alive());
    assert!(player2.alive());
    assert!(consort.alive());
    assert!(player4.alive());

    assert_contains!(
        player1.get_messages(),
        ChatMessageVariant::MartyrFailed
    );
}

#[test]
fn martyr_healed() {
    kit::scenario!(game in Night 1 where
        martyr: Martyr,
        player1: Mafioso,
        player2: Sheriff,
        doctor: Doctor,
        player4: Sheriff
    );

    assert_contains!(
        player1.get_messages(),
        ChatMessageVariant::MartyrRevealed { martyr: martyr.index() }
    );

    martyr.set_night_target(martyr);
    doctor.set_night_target(martyr);

    game.next_phase();

    assert!(martyr.alive());
    assert!(!martyr.role_state().clone().get_won_game(&game, martyr.player_ref()));
    assert!(player1.alive());
    assert!(player2.alive());
    assert!(doctor.alive());
    assert!(player4.alive());

    assert_contains!(
        player1.get_messages(),
        ChatMessageVariant::MartyrFailed
    );
}

#[test]
fn deputy_fails(){
    kit::scenario!(game in Discussion 2 where
        deputy: Deputy,
        player1: Mafioso,
        player2: Sheriff
    );

    assert!(deputy.day_target(player2));
    assert!(!deputy.alive());
    assert!(!player2.alive());
    assert!(player1.alive());
    assert!(player1.get_won_game());
    assert!(game.game_is_over());
}