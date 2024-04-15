mod kit;
use std::vec;

pub(crate) use kit::{assert_contains, assert_not_contains};

use mafia_server::game::role::{bouncer::Bouncer, engineer::Engineer, minion::Minion, zealot::Zealot};
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

        detective::Detective,
        lookout::Lookout,
        spy::{Spy, SpyBug},
        tracker::Tracker,
        philosopher::Philosopher,
        psychic::Psychic,

        doctor::Doctor,
        bodyguard::Bodyguard,
        cop::Cop,

        vigilante::Vigilante,
        veteran::Veteran,
        deputy::Deputy,

        escort::Escort,
        medium::Medium,
        retributionist::Retributionist,

        godfather::Godfather,
        mafioso::Mafioso,
        
        hypnotist::Hypnotist,
        blackmailer::Blackmailer,
        informant::Informant,
        witch::Witch,
        necromancer::Necromancer,

        janitor::Janitor,
        framer::Framer,

        jester::Jester,
        hater::Hater,
        doomsayer::{Doomsayer, DoomsayerGuess},

        arsonist::Arsonist,
        ojo::{Ojo, OjoAction},

        death::Death,

        apostle::Apostle,
        martyr::Martyr,
        wild_card::Wildcard
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
        townie: Detective,
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
        sher: Detective,
        mafia: Mafioso,
        townie: Detective
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
        sher: Detective,
        mafia: Godfather
    );
    sher.set_night_targets(vec![mafia]);
    
    game.skip_to(PhaseType::Obituary, 2);
    assert_contains!(sher.get_messages(), ChatMessageVariant::SheriffResult { suspicious: false });
}

#[test]
fn seer_basic() {
    kit::scenario!(game in Night 1 where
        philosopher: Philosopher,
        mafia1: Mafioso,
        mafia2: Informant,
        townie1: Detective,
        townie2: Vigilante,
        jester: Jester
    );
    philosopher.set_night_targets(vec![mafia1, townie1]);
    
    game.skip_to(PhaseType::Obituary, 2);
    assert_contains!(
        philosopher.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1}),
        ChatMessageVariant::SeerResult { enemies: true }
    );

    game.skip_to(PhaseType::Night, 2);
    philosopher.set_night_targets(vec![mafia1, mafia2]);
    
    game.skip_to(PhaseType::Obituary, 3);
    assert_contains!(
        philosopher.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 2}),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 3);
    philosopher.set_night_targets(vec![jester, mafia2]);
    
    game.skip_to(PhaseType::Obituary, 4);
    assert_contains!(
        philosopher.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 3}),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 4);
    philosopher.set_night_targets(vec![townie2, jester]);
    
    game.skip_to(PhaseType::Obituary, 5);
    assert_contains!(
        philosopher.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 4}),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(PhaseType::Night, 5);
    philosopher.set_night_targets(vec![townie2, townie1]);
    
    game.skip_to(PhaseType::Obituary, 6);
    assert_contains!(
        philosopher.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 5}),
        ChatMessageVariant::SeerResult { enemies: false }
    );
}

#[test]
fn jester_basic() {
    kit::scenario!(game in Night 1 where
        jester: Jester,
        townie: Detective,
        mafia: Godfather,
        lookout1: Lookout,
        lookout2: Lookout,
        mafia2: Mafioso
    );

    game.skip_to(PhaseType::Nomination, 2);
    mafia.vote_for_player(Some(jester));
    townie.vote_for_player(Some(jester));
    lookout1.vote_for_player(Some(jester));
    lookout2.vote_for_player(Some(jester));


    game.skip_to(PhaseType::Judgement, 2);
    townie.set_verdict(mafia_server::game::verdict::Verdict::Guilty);
    mafia.set_verdict(mafia_server::game::verdict::Verdict::Guilty);
    mafia2.set_verdict(mafia_server::game::verdict::Verdict::Guilty);
    lookout1.set_verdict(mafia_server::game::verdict::Verdict::Innocent);
    lookout2.set_verdict(mafia_server::game::verdict::Verdict::Innocent);

    game.skip_to(PhaseType::Night, 2);
    assert!(!jester.alive());
    lookout1.set_night_target(townie);
    lookout2.set_night_target(mafia);


    game.skip_to(PhaseType::Obituary, 3);

    assert_eq!(
        PlayerReference::all_players(&game)
            .filter(|p|!p.alive(&game)).count(), 2
    );
    assert_contains!(
        lookout1.get_messages(), 
        ChatMessageVariant::LookoutResult { players: vec![] }
    );
    assert_contains!(
        lookout2.get_messages(), 
        ChatMessageVariant::LookoutResult { players: vec![] }
    );
}

#[test]
fn psychic_auras(){
    for _ in 0..100 {
        kit::scenario!(game in Night 1 where
            psy: Psychic,
            god: Godfather,
            maf: Framer,
            town1: Detective,
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
        bugged: Detective,
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
        townie: Detective
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
        town1: Detective,
        town2: Detective
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
        philosopher: Philosopher,
        _mafioso: Mafioso,
        framer: Framer,
        town1: Detective,
        town2: Detective
    );
    assert!(trans.set_night_targets(vec![town1, town2]));
    assert!(framer.set_night_targets(vec![town1, philosopher]));
    assert!(philosopher.set_night_targets(vec![town1, town2]));
    assert!(town1.set_night_targets(vec![town2]));
    assert!(town2.set_night_targets(vec![town1]));

    game.skip_to(PhaseType::Obituary, 2);
    assert_contains!(
        philosopher.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1 }),
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
        t1: Detective,
        t2: Detective
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
        _townie: Detective,
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
        sher1: Detective,
        sher2: Detective,
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
        sher: Detective,
        informant: Informant,
        mafioso: Mafioso,
        vigilante: Vigilante
    );
    
    mafioso.set_night_target(sher);
    game.skip_to(PhaseType::Night, 2);
    vigilante.set_night_target(informant);
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
        sher: Detective,
        informant: Informant,
        mafioso: Mafioso,
        philosopher: Philosopher
    );

    assert!(witch.set_night_targets(vec![sher, mafioso]));
    game.next_phase();
    assert_contains!(witch.get_messages(), ChatMessageVariant::TargetsMessage{message: Box::new(
        ChatMessageVariant::SheriffResult{ suspicious: true }
    )});
    
    game.skip_to(PhaseType::Night, 2);
    assert!(philosopher.set_night_targets(vec![sher, informant]));
    assert!(witch.set_night_targets(vec![philosopher, mafioso]));
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
        crus: Cop,
        protected: Jester,
        townie1: Detective,
        townie2: Detective,
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
        crus: Cop,
        protected: Jester,
        townie: Detective,
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
fn veteran_basic(){
    kit::scenario!(game in Night 1 where
        vet: Veteran,
        townie: Lookout,
        _godfather: Godfather,
        mafioso: Mafioso,
        tracker: Tracker
    );

    assert!(vet.set_night_targets(vec![vet]));
    assert!(mafioso.set_night_targets(vec![vet]));
    assert!(townie.set_night_targets(vec![vet]));
    assert!(tracker.set_night_targets(vec![vet]));

    game.next_phase();

    assert!(vet.alive());
    assert!(!mafioso.alive());
    assert!(!townie.alive());

    assert_contains!(
        townie.get_messages(),
        ChatMessageVariant::LookoutResult { players: vec![mafioso.index(), tracker.index()] }
    );
    assert_contains!(
        tracker.get_messages(),
        ChatMessageVariant::TrackerResult { players: vec![] }
    );

    game.skip_to(PhaseType::Night, 2);
    assert!(vet.set_night_targets(vec![vet]));
    
    game.skip_to(PhaseType::Night, 3);
    assert!(vet.set_night_targets(vec![vet]));
    
    game.skip_to(PhaseType::Night, 4);
    assert!(!vet.set_night_targets(vec![vet]));
}

#[test]
fn veteran_does_not_kill_framed_player(){
    kit::scenario!(game in Night 1 where
        vet: Veteran,
        townie: Detective,
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
        target: Detective,
        mafioso: Mafioso,
        exe: Hater
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
        exe: Hater
    );
    let RoleState::Jester(_) = exe.role_state() else {panic!()};
}

#[test]
fn can_type_in_jail() {
    kit::scenario!(game in Dusk 1 where
        jailor: Jailor,
        detective: Detective
    );

    jailor.day_target(detective);
    game.next_phase();

    detective.send_message("Hello!");
    
    assert_contains!(jailor.get_messages(), 
        ChatMessageVariant::Normal { 
            message_sender: MessageSender::Player { player: detective.index() }, 
            text: "Hello!".to_string()
        }
    );
    
    assert_contains!(detective.get_messages(), 
        ChatMessageVariant::Normal { 
            message_sender: MessageSender::Player { player: detective.index() }, 
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
        _seer: Philosopher,
        townie: Detective,
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
 
        townie_a: Detective,
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
 
        townie_a: Detective,
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
 
        townie_a: Detective,
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
        townie: Detective
    );

    assert!(mafioso.set_night_target(townie));
    assert!(vigilante.set_night_target(townie));
    game.next_phase();
    assert_eq!(
        *game.graves.first().unwrap(), 
        Grave {
            player: townie.player_ref(),
        
            role: GraveRole::Role(Role::Detective),
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
        hypnotist: Hypnotist,
        _vigi: Vigilante
    );

    assert!(godfather.day_target(blackmailer));
    assert!(blackmailer.get_player_tags().get(&blackmailer.player_ref()).expect("blackmailer doesnt have tag").contains(&Tag::GodfatherBackup));
    
    assert!(godfather.day_target(blackmailer));
    assert!(blackmailer.get_player_tags().get(&blackmailer.player_ref()).is_none());

    assert!(godfather.day_target(blackmailer));
    assert!(blackmailer.get_player_tags().get(&blackmailer.player_ref()).expect("blackmailer doesnt have tag").contains(&Tag::GodfatherBackup));
    
    assert!(godfather.day_target(hypnotist));
    assert!(blackmailer.get_player_tags().get(&hypnotist.player_ref()).expect("hypnotist doesnt have tag").contains(&Tag::GodfatherBackup));
    assert!(blackmailer.get_player_tags().get(&blackmailer.player_ref()).is_none());
}

#[test]
fn seer_cant_see_godfather() {
    kit::scenario!(game in Night 1 where
        philosopher: Philosopher,
        godfather: Godfather,
        mafioso: Mafioso,
        townie: Detective
    );

    assert!(philosopher.set_night_targets(vec![godfather, mafioso]));
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1 }
        ),
        ChatMessageVariant::SeerResult { enemies: false }
    );
    game.skip_to(PhaseType::Night, 2);

    assert!(philosopher.set_night_targets(vec![godfather, townie]));
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 2 }
        ),
        ChatMessageVariant::SeerResult { enemies: false }
    );
}

#[test]
fn reveler_protect_still_kill() {
    kit::scenario!(game in Night 1 where
        rev: Bouncer,
        godfather: Godfather,
        jan: Janitor,
        townie_a: Detective,
        townie_b: Detective
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
        a: Detective,
        b: Detective,
        c: Detective,
        d: Detective,
        e: Detective,
        f: Detective,
        g: Detective,
        h: Detective,
        i: Detective,
        j: Detective
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
fn arsonist_ignites_and_aura(){
    kit::scenario!(game in Night 1 where
        arso: Arsonist,
        townie: Detective,
        townie2: Detective,
        gf: Godfather,
        vigi: Vigilante,
        sher: Detective
    );

    assert!(townie.set_night_target(arso));
    assert!(arso.set_night_target(arso));
    assert!(sher.set_night_target(townie));

    game.next_phase();

    assert!(arso.alive());
    assert!(!townie.alive());
    assert!(sher.alive());
    assert!(gf.alive());
    assert!(vigi.alive());

    assert_contains!(sher.get_messages_after_last_message(
        ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 1}
    ), ChatMessageVariant::SheriffResult{ suspicious: true });

    game.skip_to(PhaseType::Night, 2);
    
    assert!(arso.set_night_target(townie2));
    assert!(sher.set_night_target(townie2));

    game.next_phase();

    assert_contains!(sher.get_messages_after_last_message(
        ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 2}
    ), ChatMessageVariant::SheriffResult{ suspicious: true });

    game.skip_to(PhaseType::Nomination, 3);

    townie2.vote_for_player(Some(arso));
    gf.vote_for_player(Some(arso));
    vigi.vote_for_player(Some(arso));

    game.skip_to(PhaseType::Judgement, 3);

    gf.set_verdict(mafia_server::game::verdict::Verdict::Guilty);

    game.skip_to(PhaseType::Night, 3);

    assert!(sher.set_night_target(townie2));

    game.next_phase();
    
    assert_contains!(sher.get_messages_after_last_message(
        ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 3}
    ), ChatMessageVariant::SheriffResult{ suspicious: false });

    
}

#[test]
fn bodyguard_gets_single_target_jailed_message() {
    kit::scenario!(game in Dusk 1 where
        bg: Bodyguard,
        jailor: Jailor,
        _maf: Mafioso,
        townie: Detective
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
        player2: Detective,
        player3: Mafioso,
        player4: Detective
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
        player2: Detective,
        hypnotist: Hypnotist,
        player4: Detective
    );

    assert_contains!(
        player1.get_messages(),
        ChatMessageVariant::MartyrRevealed { martyr: martyr.index() }
    );

    martyr.set_night_target(martyr);
    hypnotist.set_night_target(martyr);

    game.next_phase();

    assert!(martyr.alive());
    assert!(!martyr.role_state().clone().get_won_game(&game, martyr.player_ref()));
    assert!(player1.alive());
    assert!(player2.alive());
    assert!(hypnotist.alive());
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
        player2: Detective,
        doctor: Doctor,
        player4: Detective
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
        player2: Detective
    );

    assert!(deputy.day_target(player2));
    assert!(!deputy.alive());
    assert!(!player2.alive());
    assert!(player1.alive());
    assert!(player1.get_won_game());
    assert!(game.game_is_over());
}

#[test]
fn ojo_transporter(){
    kit::scenario!(game in Night 1 where
        ojo: Ojo,
        transporter: Transporter,
        player1: Philosopher,
        player2: Detective,
        player3: Philosopher,
        gf: Godfather
    );

    ojo.set_role_state(
        RoleState::Ojo(Ojo{chosen_action:OjoAction::See{role:Role::Philosopher} })
    );
    transporter.set_night_targets(vec![player1, player2]);
    game.next_phase();

    assert!(player1.alive());
    assert!(player2.alive());
    assert!(player3.alive());
    assert!(gf.alive());


    assert_contains!(
        ojo.get_messages(),
        ChatMessageVariant::OjoResult{players: vec![player2.index(), player3.index()] }
    );
}

#[test]
fn apostle_converting_trapped_player_day_later(){
    kit::scenario!(game in Night 1 where
        apostle: Apostle,
        _zealot: Zealot,
        trapped: Detective,
        engineer: Engineer
    );


    assert!(engineer.set_night_target(trapped));

    game.skip_to(PhaseType::Night, 2);

    assert!(apostle.set_night_target(trapped));

    game.next_phase();

    assert!(trapped.role_state().role() == Role::Detective);
}

#[test]
fn apostle_converting_trapped_player_same_day(){
    kit::scenario!(game in Night 2 where
        apostle: Apostle,
        _zealot: Zealot,
        trapped: Detective,
        engineer: Engineer
    );


    assert!(engineer.set_night_target(trapped));
    assert!(apostle.set_night_target(trapped));

    game.next_phase();

    assert!(trapped.role_state().role() != Role::Detective);
}

#[test]
fn godfather_dies_to_veteran(){
    kit::scenario!(game in Night 1 where
        vet: Veteran,
        gf: Godfather,
        _maf: Janitor
    );

    assert!(gf.set_night_target(vet));
    assert!(vet.set_night_target(vet));

    game.next_phase();

    assert!(!gf.alive());
    assert!(vet.alive());
}

#[test]
fn godfather_dies_to_veteran_after_possessed(){
    kit::scenario!(game in Night 1 where
        vet: Veteran,
        gf: Godfather,
        _maf: Janitor,
        min: Minion
    );

    assert!(min.set_night_targets(vec![gf, vet]));
    assert!(vet.set_night_target(vet));

    game.next_phase();

    assert!(!gf.alive());
    assert!(vet.alive());
    assert!(min.alive());
}