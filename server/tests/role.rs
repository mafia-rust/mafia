mod kit;
use std::vec;

pub(crate) use kit::{assert_contains, assert_not_contains};

use mafia_server::game::{components::cult::CultAbility, role::{armorsmith::Armorsmith, flower_girl::FlowerGirl, scarecrow::Scarecrow}};
pub use mafia_server::game::{
    chat::{ChatMessageVariant, MessageSender, ChatGroup}, 
    grave::*, 
    role_list::Faction,
    player::PlayerReference,
    tag::Tag,
    verdict::Verdict,
    role::{
        Role,
        RoleState,

        jailor::Jailor,
        
        detective::Detective,
        snoop::Snoop,
        lookout::Lookout,
        spy::{Spy, SpyBug},
        tracker::Tracker,
        philosopher::Philosopher,
        psychic::Psychic,
        gossip::Gossip, 
        
        doctor::Doctor,
        bodyguard::Bodyguard,
        cop::Cop,
        bouncer::Bouncer,
        engineer::Engineer,

        vigilante::Vigilante,
        veteran::Veteran,
        deputy::Deputy,
        marksman::Marksman, 
        
        transporter::Transporter,
        escort::Escort,
        mayor::Mayor,
        medium::Medium,
        retributionist::Retributionist,

        godfather::Godfather,
        mafioso::Mafioso,
        
        framer::Framer,
        hypnotist::Hypnotist,
        blackmailer::Blackmailer,
        informant::Informant,
        witch::Witch,
        necromancer::Necromancer,
        mortician::Mortician,
        mafia_support_wildcard::MafiaSupportWildcard, 
        

        jester::Jester,
        rabble_rouser::RabbleRouser,
        minion::Minion,
        politician::Politician,
        doomsayer::{Doomsayer, DoomsayerGuess},
        death::Death,
        wild_card::Wildcard,
        martyr::Martyr,

        apostle::Apostle,
        zealot::Zealot,
        
        arsonist::Arsonist,
        ojo::{Ojo, OjoAction},
        pyrolisk::Pyrolisk,
        puppeteer::{Puppeteer, PuppeteerAction},
        fiends_wildcard::FiendsWildcard, 
    }, 
    phase::{
        PhaseState, 
        PhaseType::{self, *}
    }
};
// Pub use so that submodules don't have to reimport everything.
pub use mafia_server::packet::ToServerPacket;

#[test]
fn medium_receives_dead_messages_from_jail() {
    kit::scenario!(game in Night 2 where
        medium: Medium,
        jailor: Jailor,
        townie: Detective,
        mafioso: Mafioso
    );
    mafioso.set_night_selection_single(townie);
    game.skip_to(Nomination, 3);
    
    jailor.day_target(medium);

    game.skip_to(Night, 3);
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
fn detective_basic() {
    kit::scenario!(game in Night 1 where
        sher: Detective,
        mafia: Mafioso,
        townie: Detective
    );
    sher.set_night_selection(vec![mafia]);
    game.next_phase();
    assert_contains!(
        sher.get_messages_after_night(1),
        ChatMessageVariant::SheriffResult { suspicious: true }
    );

    game.skip_to(Night, 2);
    sher.set_night_selection(vec![townie]);
    game.next_phase();
    assert_contains!(
        sher.get_messages_after_night(2),
        ChatMessageVariant::SheriffResult { suspicious: false }
    );
}

#[test]
fn detective_neutrals(){
    kit::scenario!(game in Night 1 where
        sher: Detective,
        _mafia: Godfather,
        minion: Minion,
        jester: Jester,
        politician: Politician
    );

    sher.set_night_selection(vec![minion]);
    game.next_phase();
    assert_contains!(
        sher.get_messages_after_night(1),
        ChatMessageVariant::SheriffResult { suspicious: true }
    );
    
    game.skip_to(Night, 2);
    sher.set_night_selection(vec![jester]);
    game.next_phase();
    assert_contains!(
        sher.get_messages_after_night(2),
        ChatMessageVariant::SheriffResult { suspicious: false }
    );
    
    game.skip_to(Night, 3);
    sher.set_night_selection(vec![politician]);
    game.next_phase();
    assert_contains!(
        sher.get_messages_after_night(3),
        ChatMessageVariant::SheriffResult { suspicious: true }
    );

}

#[test]
fn mortician_obscures_on_stand(){
    kit::scenario!(game in Night 1 where
        mortician: Mortician,
        townie: Detective,
        jail: Jailor,
        gf: Godfather
    );

    mortician.set_night_selection_single(townie);
    game.skip_to(Nomination, 2);
    
    jail.vote_for_player(Some(townie));
    gf.vote_for_player(Some(townie));
    mortician.vote_for_player(Some(townie));

    game.skip_to(Judgement, 2);
    jail.set_verdict(Verdict::Guilty);

    game.skip_to(Night, 2);
    
    assert_eq!(game.graves[0].information, GraveInformation::Obscured);
    assert_contains!(mortician.get_messages(), ChatMessageVariant::PlayerRoleAndAlibi { player: townie.player_ref(), role: Role::Detective, will: "".to_string() });
}

#[test]
fn mortician_obscures_fail_after_death(){
    kit::scenario!(game in Night 1 where
        mortician: Mortician,
        townie: Detective,
        jail: Jailor,
        gf: Godfather
    );

    mortician.set_night_selection_single(townie);
    game.skip_to(Nomination, 2);
    
    jail.vote_for_player(Some(mortician));
    gf.vote_for_player(Some(mortician));
    townie.vote_for_player(Some(mortician));

    game.skip_to(Judgement, 2);
    jail.set_verdict(Verdict::Guilty);

    game.skip_to(Night, 2);
    gf.set_night_selection_single(townie);
    game.next_phase();
    assert!(matches!(game.graves[1].information, GraveInformation::Normal { role: Role::Detective, .. }));
    assert_not_contains!(mortician.get_messages(), ChatMessageVariant::PlayerRoleAndAlibi { player: townie.player_ref(), role: Role::Detective, will: "".to_string() });
}

#[test]
fn detective_godfather() {
    kit::scenario!(game in Night 1 where
        sher: Detective,
        mafia: Godfather
    );
    sher.set_night_selection(vec![mafia]);
    game.next_phase();
    assert_contains!(sher.get_messages(), ChatMessageVariant::SheriffResult { suspicious: false });
}

#[test]
fn philosopher_basic() {
    kit::scenario!(game in Night 1 where
        philosopher: Philosopher,
        mafia1: Mafioso,
        mafia2: Informant,
        townie1: Detective,
        townie2: Vigilante
    );

    philosopher.set_night_selection(vec![mafia1, townie1]);
    
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_night(1),
        ChatMessageVariant::SeerResult { enemies: true }
    );

    game.skip_to(Night, 2);
    philosopher.set_night_selection(vec![mafia1, mafia2]);
    
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_night(2),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(Night, 3);
    philosopher.set_night_selection(vec![townie2, townie1]);
    
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_night(3),
        ChatMessageVariant::SeerResult { enemies: false }
    );
}

#[test]
fn philosopher_neutrals() {
    kit::scenario!(game in Night 1 where
        philosopher: Philosopher,
        mafia1: Mafioso,
        townie1: Vigilante,
        jester: Jester,
        minion: Minion
    );

    game.skip_to(Night, 3);
    philosopher.set_night_selection(vec![jester, mafia1]);
    
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_night(3),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(Night, 4);
    philosopher.set_night_selection(vec![townie1, jester]);
    
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_night(4),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(Night, 6);
    philosopher.set_night_selection(vec![townie1, minion]);
    
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_night(6),
        ChatMessageVariant::SeerResult { enemies: true }
    );

    game.skip_to(Night, 7);
    philosopher.set_night_selection(vec![jester, minion]);
    
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_night(7),
        ChatMessageVariant::SeerResult { enemies: false }
    );

    game.skip_to(Night, 8);
    philosopher.set_night_selection(vec![mafia1, minion]);
    
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_night(8),
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

    game.skip_to(Nomination, 2);
    mafia.vote_for_player(Some(jester));
    townie.vote_for_player(Some(jester));
    lookout1.vote_for_player(Some(jester));
    lookout2.vote_for_player(Some(jester));


    game.skip_to(Judgement, 2);
    townie.set_verdict(mafia_server::game::verdict::Verdict::Guilty);
    mafia.set_verdict(mafia_server::game::verdict::Verdict::Guilty);
    mafia2.set_verdict(mafia_server::game::verdict::Verdict::Guilty);
    lookout1.set_verdict(mafia_server::game::verdict::Verdict::Innocent);
    lookout2.set_verdict(mafia_server::game::verdict::Verdict::Innocent);

    game.skip_to(Night, 2);
    assert!(!jester.alive());
    lookout1.set_night_selection_single(townie);
    lookout2.set_night_selection_single(mafia);


    game.skip_to(Obituary, 3);

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
fn rabble_rouser_dies(){
    kit::scenario!(game in Night 1 where
        exe: RabbleRouser,
        townie: Detective,
        mafioso: Mafioso
    );

    game.skip_to(Nomination, 2);
    mafioso.vote_for_player(Some(townie));
    exe.vote_for_player(Some(townie));

    game.skip_to(Judgement, 2);
    exe.set_verdict(Verdict::Guilty);
    mafioso.set_verdict(Verdict::Guilty);

    game.skip_to(FinalWords, 2);
    assert!(!exe.alive());
    assert!(townie.alive());
    assert!(mafioso.alive());
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
        let messages = psy.get_messages_after_night(1);
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

        game.skip_to(Night, 2);
        maf.set_night_selection_single(town1);
        game.next_phase();
        let messages = psy.get_messages_after_night(2);
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
fn flower_girl_basic(){
    kit::scenario!(game in Nomination 2 where
        fg: FlowerGirl,
        townie: Detective,
        mafioso: Mafioso
    );

    fg.vote_for_player(Some(townie));
    mafioso.vote_for_player(Some(townie));

    game.skip_to(Judgement, 2);

    fg.set_verdict(Verdict::Guilty);
    mafioso.set_verdict(Verdict::Guilty);
    
    game.skip_to(Obituary, 3);
    assert_contains!(
        fg.get_messages_after_night(1),
        ChatMessageVariant::FlowerGirlResult { evil_count: 1 }
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
        bugged: Detective,
        jester: Jester,
        witch: Witch
    );
    spy.set_night_selection_single(jester);
    transp.set_night_selection(vec![jester, bugged]);
    blackmailer.set_night_selection_single(jester);
    esc.set_night_selection_single(jester);
    witch.set_night_selection(vec![jester, esc]);

    game.next_phase();

    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyBug { bug: SpyBug::Silenced });
    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyBug { bug: SpyBug::Roleblocked });
    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyBug { bug: SpyBug::Transported });
    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyBug { bug: SpyBug::Possessed });

    
    assert_contains!(spy.get_messages(), ChatMessageVariant::SpyMafiaVisit { players: vec![bugged.index(), bugged.index()] });
}

#[test]
fn bodyguard_basic() {
    kit::scenario!(game in Night 2 where
        maf: Mafioso,
        bg: Bodyguard,
        townie: Detective
    );

    maf.set_night_selection_single(townie);
    bg.set_night_selection_single(townie);

    game.skip_to(Obituary, 3);

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
    trans.set_night_selection(vec![town1, town2]);
    vigi.set_night_selection_single(town1);
    escort.set_night_selection_single(town2);

    game.skip_to(Obituary, 3);
    assert!(town1.alive());
    assert!(!town2.alive());

    assert!(town1.was_roleblocked());
    assert!(!town2.was_roleblocked());
    
    game.skip_to(Obituary, 4);
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
    assert!(trans.set_night_selection(vec![town1, town2]));
    assert!(framer.set_night_selection(vec![town1, philosopher]));
    assert!(philosopher.set_night_selection(vec![town1, town2]));
    assert!(town1.set_night_selection(vec![town2]));
    assert!(town2.set_night_selection(vec![town1]));

    game.skip_to(Obituary, 2);
    assert_contains!(
        philosopher.get_messages_after_night(1),
        ChatMessageVariant::SeerResult { enemies: true }
    );
    assert_contains!(
        town1.get_messages_after_night(1),
        ChatMessageVariant::SheriffResult { suspicious: false }
    );
    assert_contains!(
        town2.get_messages_after_night(1),
        ChatMessageVariant::SheriffResult { suspicious: true }
    );
}

/// Test that the bodyguard protects the person their target was swapped with
#[test]
fn bodyguard_protects_transported_target() {
    kit::scenario!(game in Night 2 where
        trans: Transporter,
        maf: Mafioso,
        bg: Bodyguard,
        t1: Detective,
        t2: Detective
    );
    trans.set_night_selection(vec![t1, t2]);
    maf.set_night_selection_single(t1);
    bg.set_night_selection_single(t1);
    
    game.skip_to(Obituary, 3);
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

    game.skip_to(Nomination, 2);
    mayor.vote_for_player(Some(mafioso));
    mayor.day_target(mayor);
    assert_eq!(game.current_phase().phase(), Testimony);
}


#[test]
fn retributionist_basic(){
    kit::scenario!(game in Night 2 where
        ret: Retributionist,
        sher1: Detective,
        sher2: Detective,
        mafioso: Mafioso
    );

    mafioso.set_night_selection_single(sher1);
    game.skip_to(Night, 3);
    mafioso.set_night_selection_single(sher2);
    game.skip_to(Night, 4);

    assert!(!sher1.alive());
    assert!(!sher2.alive());

    assert!(ret.set_night_selection(vec![sher1, mafioso]));
    game.next_phase();
    assert_contains!(
        ret.get_messages_after_night(4),
        ChatMessageVariant::TargetsMessage{message: Box::new(
            ChatMessageVariant::SheriffResult{ suspicious: true }
        )}
    );

    game.skip_to(Night, 5);
    assert!(ret.set_night_selection(vec![sher1, mafioso]));
    game.next_phase();
    assert_contains!(
        ret.get_messages_after_night(5),
        ChatMessageVariant::TargetsMessage{message: Box::new(
            ChatMessageVariant::SheriffResult{ suspicious: true }
        )}
    );

    game.skip_to(Night, 6);
    assert!(!ret.set_night_selection(vec![sher1, mafioso]));
    game.next_phase();
    assert_not_contains!(
        ret.get_messages_after_night(6),
        ChatMessageVariant::TargetsMessage{message: Box::new(
            ChatMessageVariant::SheriffResult{ suspicious: true }
        )}
    );
}

#[test]
fn necromancer_basic(){
    kit::scenario!(game in Night 2 where
        ret: Necromancer,
        sher: Detective,
        informant: Informant,
        mafioso: Mafioso,
        vigilante: Vigilante
    );
    
    mafioso.set_night_selection_single(sher);
    game.skip_to(Night, 3);
    vigilante.set_night_selection_single(informant);
    game.skip_to(Night, 4);



    assert!(ret.set_night_selection(vec![sher, mafioso]));
    game.next_phase();
    assert_contains!(
        ret.get_messages_after_night(3),
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

    assert!(witch.set_night_selection(vec![sher, mafioso]));
    game.next_phase();
    assert_contains!(witch.get_messages(), ChatMessageVariant::TargetsMessage{message: Box::new(
        ChatMessageVariant::SheriffResult{ suspicious: true }
    )});
    
    game.skip_to(Night, 2);
    assert!(philosopher.set_night_selection(vec![sher, informant]));
    assert!(witch.set_night_selection(vec![philosopher, mafioso]));
    game.next_phase();
    assert_contains!(
        witch.get_messages_after_night(2),
        ChatMessageVariant::TargetsMessage{message: Box::new(
            ChatMessageVariant::SeerResult { enemies: false }
        )}
    );
}

#[test]
fn cop_basic(){
    kit::scenario!(game in Night 2 where
        crus: Cop,
        protected: Jester,
        townie1: Detective,
        townie2: Detective,
        mafioso: Mafioso
    );

    crus.set_night_selection(vec![protected]);
    townie1.set_night_selection(vec![protected]);
    townie2.set_night_selection(vec![protected]);
    mafioso.set_night_selection(vec![protected]);

    game.skip_to(Night, 3);

    assert!(crus.alive());
    assert!(protected.alive());
    assert!(townie1.alive());
    assert!(townie2.alive());
    assert!(!mafioso.alive());

    crus.set_night_selection(vec![protected]);
    townie1.set_night_selection(vec![protected]);
    townie2.set_night_selection(vec![protected]);

    game.next_phase();
    
    assert!(crus.alive());
    assert!(protected.alive());
    assert!(townie1.alive() || townie2.alive());
    assert!(!(townie1.alive() && townie2.alive()));
}

#[test]
fn cop_does_not_kill_framed_player(){
    kit::scenario!(game in Night 2 where
        crus: Cop,
        protected: Jester,
        townie: Detective,
        framer: Framer,
        mafioso: Mafioso
    );

    assert!(crus.set_night_selection(vec![protected]));
    assert!(framer.set_night_selection(vec![townie, protected]));

    game.next_phase();

    assert!(crus.alive());
    assert!(protected.alive());
    assert!(framer.alive());
    assert!(mafioso.alive());
    assert!(townie.alive());
}

#[test]
fn veteran_basic(){
    kit::scenario!(game in Night 2 where
        vet: Veteran,
        townie: Lookout,
        _godfather: Godfather,
        framer: Framer,
        tracker: Tracker
    );

    assert!(vet.set_night_selection(vec![vet]));
    assert!(framer.set_night_selection(vec![vet]));
    assert!(townie.set_night_selection(vec![vet]));
    assert!(tracker.set_night_selection(vec![vet]));

    game.next_phase();

    assert!(vet.alive());
    assert!(!framer.alive());
    assert!(!townie.alive());

    assert!(
        townie.get_messages().contains(&ChatMessageVariant::LookoutResult { players: vec![framer.index(), tracker.index()] }) ||
        townie.get_messages().contains(&ChatMessageVariant::LookoutResult { players: vec![tracker.index(), framer.index()] })
    );

    // assert_contains!(
    //     townie.get_messages(),
    //     ChatMessageVariant::LookoutResult { players: vec![framer.index(), tracker.index()] }
    // );
    assert_contains!(
        tracker.get_messages(),
        ChatMessageVariant::TrackerResult { players: vec![] }
    );

    game.skip_to(Night, 3);
    assert!(vet.set_night_selection(vec![vet]));
    
    game.skip_to(Night, 4);
    assert!(vet.set_night_selection(vec![vet]));
    
    game.skip_to(Night, 5);
    assert!(!vet.set_night_selection(vec![vet]));
}

#[test]
fn veteran_does_not_kill_framed_player(){
    kit::scenario!(game in Night 2 where
        vet: Veteran,
        townie: Detective,
        framer: Framer,
        mafioso: Mafioso
    );

    assert!(vet.set_night_selection(vec![vet]));
    assert!(framer.set_night_selection(vec![townie, vet]));

    game.next_phase();

    assert!(vet.alive());
    assert!(framer.alive());
    assert!(mafioso.alive());
    assert!(townie.alive());
}

#[test]
fn rabble_rouser_turns_into_jester(){
    kit::scenario!(game in Night 2 where
        target: Detective,
        mafioso: Mafioso,
        exe: RabbleRouser
    );

    assert!(mafioso.set_night_selection(vec![target]));

    game.skip_to(Nomination, 3);

    assert!(!target.alive());
    assert!(exe.alive());
    assert!(mafioso.alive());
    let RoleState::Jester(_) = exe.role_state() else {panic!()};
}
#[test]
fn rabble_rouser_instantly_turns_into_jester(){
    kit::scenario!(_game where
        exe: RabbleRouser
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
        mortician: Mortician
    );

    mafioso.set_night_selection_single(mortician);

    game.next_phase();

    assert!(mortician.alive());
}

#[test]
fn marksman_basic() {
    kit::scenario!(game in Night 2 where
        mk: Marksman,
        dt: Detective,
        gf: Godfather
    );

    assert!(dt.set_night_selection_single(gf));
    assert!(mk.day_target(dt));
    assert!(mk.set_night_selection(vec![dt, gf]));

    game.next_phase();

    assert!(!dt.alive());
    assert!(gf.alive());
    assert!(mk.alive());
}

#[test]
fn transporter_cant_transport_dead() {
    kit::scenario!(game in Night 2 where
        mafioso: Mafioso,
        _vet: Veteran,
        _necro: Necromancer,
        _seer: Philosopher,
        townie: Detective,
        thomas: Jailor,
        trans: Transporter
    );

    mafioso.set_night_selection_single(thomas);

    game.next_phase();

    assert!(!thomas.alive());

    game.skip_to(Night, 3);

    assert!(trans.set_night_selection_single(townie));
    assert!(!trans.set_night_selection(vec![townie, thomas]), "Transporter targeted dead player");

    game.next_phase();

    assert_not_contains!(thomas.get_messages(), ChatMessageVariant::Transported);
    assert_not_contains!(townie.get_messages(), ChatMessageVariant::Transported);
}

#[test]
fn double_transport() {
    kit::scenario!(game in Night 2 where
        mafioso: Mafioso,
 
        townie_a: Detective,
        townie_b: Jailor,

        trans_a: Transporter,
        trans_b: Transporter
    );
    
    assert!(mafioso.set_night_selection_single(townie_a));

    assert!(trans_a.set_night_selection(vec![townie_a, townie_b]));
    assert!(trans_b.set_night_selection(vec![townie_b, townie_a]));

    game.next_phase();
    assert!(!townie_a.alive());
    assert!(townie_b.alive());
}


#[test]
fn double_transport_single_player() {
    kit::scenario!(game in Night 2 where
        mafioso: Mafioso,
 
        townie_a: Detective,
        townie_b: Jailor,
        townie_c: Vigilante,

        trans_a: Transporter,
        trans_b: Transporter
    );
    
    assert!(mafioso.set_night_selection_single(townie_a));

    assert!(trans_a.set_night_selection(vec![townie_a, townie_b]));
    assert!(trans_b.set_night_selection(vec![townie_a, townie_c]));


    game.next_phase();
    assert!(townie_a.alive());
    assert!(!townie_b.alive());
    assert!(townie_c.alive());
}

#[test]
fn double_transport_three_players() {
    kit::scenario!(game in Night 2 where
        mafioso: Mafioso,
 
        townie_a: Detective,
        townie_b: Jailor,
        townie_c: Vigilante,

        trans_a: Transporter,
        trans_b: Transporter,
        trans_c: Transporter
    );
    
    assert!(mafioso.set_night_selection_single(townie_a));

    assert!(trans_a.set_night_selection(vec![townie_a, townie_b]));
    assert!(trans_b.set_night_selection(vec![townie_a, townie_c]));
    assert!(trans_c.set_night_selection(vec![townie_b, townie_c]));


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

    assert!(mafioso.set_night_selection_single(townie));
    assert!(vigilante.set_night_selection_single(townie));
    game.next_phase();
    assert_eq!(
        *game.graves.first().unwrap(),
        Grave{ 
            player: townie.player_ref(),
            died_phase: GravePhase::Night,
            day_number: 2,
            information: GraveInformation::Normal{
                role: Role::Detective,
                death_cause: GraveDeathCause::Killers(vec![GraveKiller::Faction(Faction::Mafia), GraveKiller::Role(Role::Vigilante)]),
                will: "".to_string(),
                death_notes: vec![],
            }
        }
    )
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

    assert!(mafioso.set_night_selection_single(townie_b));
    assert!(vigilante.set_night_selection_single(townie_b));
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
            died_phase: GravePhase::Night,
            day_number: 2,
            information: GraveInformation::Normal{
                role: Role::Doctor,
                death_cause: GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Doomsayer), GraveKiller::Faction(Faction::Mafia), GraveKiller::Role(Role::Vigilante)]),
                will: "".to_string(),
                death_notes: vec![],
            }
        }
    );
}
#[test]
fn vigilante_cant_select_night_one() {
    kit::scenario!(game in Night 1 where
        townie_b: Doctor,
        _godfather: Godfather,
        vigilante_suicide: Vigilante

    );
    assert!(!vigilante_suicide.set_night_selection_single(townie_b));
}

#[test]
fn godfather_backup_kills_esc() {
    kit::scenario!(game in Night 2 where
        godfather: Godfather,
        hypnotist: Hypnotist,
        det: Detective,
        esc: Escort
    );

    assert!(godfather.day_target(hypnotist));

    assert!(hypnotist.set_night_selection_single(det));
    assert!(esc.set_night_selection_single(godfather));

    game.next_phase();
    assert!(!det.alive());
    assert!(godfather.alive());
    assert!(hypnotist.alive());
    assert!(esc.alive());
}

#[test]
fn snoop_basic() {
    kit::scenario!(game in Night 1 where
        gf: Godfather,
        det: Detective,
        snoop: Snoop
    );

    assert!(snoop.set_night_selection_single(det));
    assert!(det.set_night_selection_single(snoop));
    game.next_phase();
    assert_contains!(
        snoop.get_messages(),
        ChatMessageVariant::SnoopResult { townie: false }
    );

    game.skip_to(Night, 2);

    assert!(snoop.set_night_selection_single(det));
    game.next_phase();
    assert_contains!(
        snoop.get_messages(),
        ChatMessageVariant::SnoopResult { townie: true }
    );

    game.skip_to(Night, 3);

    assert!(snoop.set_night_selection_single(gf));
    game.next_phase();
    assert_contains!(
        snoop.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 3 }
        ),
        ChatMessageVariant::SnoopResult { townie: false }
    );
}

#[test]
fn godfather_backup_kills_jail() {
    kit::scenario!(game in Dusk 2 where
        godfather: Godfather,
        hypnotist: Hypnotist,
        det: Detective,
        jail: Jailor
    );

    assert!(jail.day_target(godfather));
    assert!(godfather.day_target(hypnotist));

    game.next_phase();
    assert!(hypnotist.set_night_selection_single(det));

    game.next_phase();

    assert!(!det.alive());
    assert!(godfather.alive());
    assert!(hypnotist.alive());
    assert!(jail.alive());
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
fn gossip_basic_friends() {
    kit::scenario!(game in Night 1 where
        gossip: Gossip,
        t1: Detective,
        t2: Detective,
        _gf: Godfather
    );

    assert!(gossip.set_night_selection_single(t1));
    assert!(t1.set_night_selection_single(t2));

    game.next_phase();

    assert_contains!(
        gossip.get_messages(),
        ChatMessageVariant::GossipResult { enemies: false }
    );
}

#[test]
fn gossip_basic_enemies_inverted() {
    kit::scenario!(game in Night 1 where
        gossip: Gossip,
        t1: Detective,
        _t2: Detective,
        py: Pyrolisk
    );

    assert!(gossip.set_night_selection_single(py));
    assert!(py.set_night_selection_single(t1));

    game.next_phase();

    assert_contains!(
        gossip.get_messages(),
        ChatMessageVariant::GossipResult { enemies: true }
    );
}

#[test]
fn gossip_basic_enemies() {
    kit::scenario!(game in Night 1 where
        gossip: Gossip,
        t1: Detective,
        _t2: Detective,
        py: Pyrolisk
    );

    assert!(gossip.set_night_selection_single(t1));
    assert!(t1.set_night_selection_single(py));

    game.next_phase();

    assert_contains!(
        gossip.get_messages(),
        ChatMessageVariant::GossipResult { enemies: true }
    );
}

#[test]
fn gossip_framer() {
    kit::scenario!(game in Night 1 where 
        gossip: Gossip,
        framer: Framer,
        t2: Detective,
        townie: Detective,
        _gf: Godfather    
    );

    assert!(gossip.set_night_selection_single(townie));
    assert!(townie.set_night_selection_single(framer));

    game.next_phase();

    assert_contains!(
        gossip.get_messages(),
        ChatMessageVariant::GossipResult { enemies: true }
    );

    game.skip_to(Night, 2);

    assert!(gossip.set_night_selection_single(townie));
    assert!(townie.set_night_selection_single(framer));
    assert!(framer.set_night_selection(vec![townie, gossip]));

    game.next_phase();

    assert_contains!(
        gossip.get_messages(),
        ChatMessageVariant::GossipResult { enemies: false }
    );

    game.skip_to(Night, 3);

    assert!(gossip.set_night_selection_single(t2));
    assert!(t2.set_night_selection_single(townie));
    assert!(framer.set_night_selection(vec![townie, gossip]));

    game.next_phase();

    assert_contains!(
        gossip.get_messages_after_night(3),
        ChatMessageVariant::GossipResult { enemies: true }
    );

    game.skip_to(Night, 4);

    assert!(gossip.set_night_selection_single(t2));
    assert!(t2.set_night_selection_single(townie));

    game.next_phase();

    assert_contains!(
        gossip.get_messages_after_night(4),
        ChatMessageVariant::GossipResult { enemies: false }
    );
}

#[test]
fn seer_cant_see_godfather() {
    kit::scenario!(game in Night 1 where
        philosopher: Philosopher,
        godfather: Godfather,
        mafioso: Mafioso,
        townie: Detective
    );

    assert!(philosopher.set_night_selection(vec![godfather, mafioso]));
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1 }
        ),
        ChatMessageVariant::SeerResult { enemies: false }
    );
    game.skip_to(Night, 2);

    assert!(philosopher.set_night_selection(vec![godfather, townie]));
    game.next_phase();
    assert_contains!(
        philosopher.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 2 }
        ),
        ChatMessageVariant::SeerResult { enemies: false }
    );
}

#[test]
fn bouncer_jailor_double_block() {
    kit::scenario!(game in Dusk 1 where
        b: Bouncer,
        jail: Jailor,
        gf: Godfather,
        det: Detective
    );

    jail.day_target(gf);

    game.next_phase();

    det.set_night_selection_single(gf);
    b.set_night_selection_single(gf);

    game.next_phase();

    assert_contains!(
        det.get_messages_after_last_message(ChatMessageVariant::PhaseChange { phase: PhaseState::Night, day_number: 1 }),
        ChatMessageVariant::Wardblocked
    );
}

#[test]
fn bouncer_ojo_block() {
    kit::scenario!(game in Night 2 where
        b: Bouncer,
        ojo: Ojo,
        det1: Detective,
        det2: Detective,
        det3: Detective,
        det4: Detective
    );

    ojo.set_role_state(RoleState::Ojo(Ojo{
        chosen_action: OjoAction::Kill { role: Role::Detective }
    }));
    b.set_night_selection_single(det1);

    game.next_phase();

    assert!(det1.alive());
    assert!(det2.alive());
    assert!(det3.alive());
    assert!(det4.alive());
}

#[test]
fn godfather_backup_sets_off_engineer_trap() {
    kit::scenario!(game in Night 2 where
        gf: Godfather,
        backup: Framer,
        eng: Engineer,
        esc: Escort
    );

    assert!(gf.day_target(backup));
    assert!(backup.set_night_selection_single(esc));
    assert!(esc.set_night_selection_single(gf));
    assert!(eng.set_night_selection_single(esc));

    game.next_phase();

    assert!(gf.alive());
    assert!(!backup.alive());
    assert!(esc.alive());
    assert!(eng.alive());
}

#[test]
fn godfather_wardblock_still_kills() {
    kit::scenario!(game in Night 2 where
        rev: Bouncer,
        godfather: Godfather,
        jan: Mortician,
        townie_a: Detective,
        townie_b: Detective
    );

    assert!(rev.set_night_selection(vec![townie_a]));
    assert!(godfather.set_night_selection(vec![townie_a]));
    assert!(godfather.day_target(jan));
    assert!(jan.set_night_selection(vec![townie_b]));

    game.next_phase();
    assert_not_contains!(
        townie_a.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 1}
        ),
        ChatMessageVariant::Wardblocked
    );
    assert_contains!(
        godfather.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 1}
        ),
        ChatMessageVariant::Wardblocked
    );
    assert_not_contains!(
        jan.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 1}
        ),
        ChatMessageVariant::Wardblocked
    );

    assert!(!townie_b.alive());
    assert!(townie_a.alive());
}

#[test]
fn cult_alternates() {
    kit::scenario!(game in Night 1 where
        apostle: Apostle,
        b: Detective,
        c: Detective,
        d: Detective,
        e: Detective,
        f: Detective,
        g: Detective
    );


    //apostle converts
    assert!(game.cult().next_ability == CultAbility::Convert);
    assert!(apostle.set_night_selection_single(b));
    game.next_phase();
    assert!(b.alive());
    assert!(b.role_state().role().faction() == Faction::Cult);

    //zealot kills, apostle waits
    game.skip_to(Night, 2);
    assert!(game.cult().next_ability == CultAbility::Kill);
    assert!(game.cult().ordered_cultists.len() == 2);
    assert!(!apostle.set_night_selection_single(d));
    assert!(b.set_night_selection_single(c));
    game.next_phase();
    assert!(!c.alive());
    assert!(d.alive());
    assert!(d.role_state().role().faction() != Faction::Cult);

    //zealot waits, apostle converts
    game.skip_to(Night, 3);
    assert!(game.cult().ordered_cultists.len() == 2);
    assert!(apostle.set_night_selection_single(d));
    assert!(!b.set_night_selection_single(e));
    game.next_phase();
    assert!(e.alive());
    assert!(d.alive());
    assert!(d.role_state().role().faction() == Faction::Cult);

    //zealot kills, apostle waits
    game.skip_to(Night, 4);
    assert!(game.cult().ordered_cultists.len() == 3);
    assert!(!apostle.set_night_selection_single(f));
    assert!(d.set_night_selection_single(g));
    game.next_phase();
    assert!(f.alive());
    assert!(!g.alive());
}

#[test]
fn puppeteer_marionettes_philosopher(){
    kit::scenario!(game in Night 1 where
        puppeteer: Puppeteer,
        philo: Philosopher,
        townie: Detective,
        townie2: Detective
    );

    puppeteer.set_role_state(RoleState::Puppeteer(Puppeteer{
        marionettes_remaining: 3,
        action: PuppeteerAction::String
    }));

    assert!(puppeteer.set_night_selection_single(townie));
    assert!(philo.set_night_selection(vec![townie2, townie]));

    game.next_phase();
    assert_contains!(
        philo.get_messages_after_night(1),
        ChatMessageVariant::SeerResult{ enemies: true }
    );

    game.skip_to(Night, 2);

    assert!(philo.set_night_selection(vec![puppeteer, townie]));

    game.next_phase();
    assert_contains!(
        philo.get_messages_after_night(2),
        ChatMessageVariant::SeerResult{ enemies: false }
    );
}

#[test]
fn puppeteer_marionettes_die(){
    kit::scenario!(game in Night 1 where
        puppeteer: Puppeteer,
        townie: Detective,
        townie2: Detective,
        townie3: Detective
    );

    puppeteer.set_role_state(RoleState::Puppeteer(Puppeteer{
        marionettes_remaining: 3,
        action: PuppeteerAction::String
    }));

    assert!(puppeteer.set_night_selection_single(townie));

    game.skip_to(Night, 2);

    assert!(puppeteer.set_night_selection_single(townie2));

    game.next_phase();

    assert!(!townie.alive());
    assert!(townie2.alive());
    assert!(townie3.alive());
    assert!(puppeteer.alive());

    game.skip_to(Obituary, 4);

    assert!(!townie.alive());
    assert!(!townie2.alive());
    assert!(townie3.alive());
    assert!(puppeteer.alive());
}

#[test]
fn puppeteer_marionettes_win(){
    kit::scenario!(game in Night 1 where
        puppeteer: Puppeteer,
        townie: Detective,
        townie2: Detective
    );

    puppeteer.set_role_state(RoleState::Puppeteer(Puppeteer{
        marionettes_remaining: 3,
        action: PuppeteerAction::String
    }));

    assert!(puppeteer.set_night_selection_single(townie));

    game.skip_to(Nomination, 2);

    puppeteer.vote_for_player(Some(townie2));
    townie.vote_for_player(Some(townie2));

    game.skip_to(Judgement, 2);

    puppeteer.set_verdict(Verdict::Guilty);

    game.skip_to(Dusk, 2);

    assert!(puppeteer.alive());
    assert!(townie.alive());
    assert!(!townie2.alive());

    assert!(puppeteer.get_won_game());
    assert!(townie.get_won_game());
    assert!(!townie2.get_won_game());
}

#[test]
fn deputy_shoots_marionette(){
    kit::scenario!(game in Night 1 where
        deputy: Deputy,
        puppeteer: Puppeteer,
        townie: Detective
    );

    puppeteer.set_role_state(RoleState::Puppeteer(Puppeteer{
        marionettes_remaining: 3,
        action: PuppeteerAction::String
    }));
    assert!(puppeteer.set_night_selection_single(townie));

    game.skip_to(Discussion, 2);

    deputy.day_target(townie);

    assert!(puppeteer.alive());
    assert!(!townie.alive());
    assert!(deputy.alive());
}

#[test]
fn vigilante_shoots_marionette(){
    kit::scenario!(game in Night 2 where
        vigilante: Vigilante,
        puppeteer: Puppeteer,
        townie: Detective
    );

    puppeteer.set_role_state(RoleState::Puppeteer(Puppeteer{
        marionettes_remaining: 3,
        action: PuppeteerAction::String
    }));
    assert!(puppeteer.set_night_selection_single(townie));
    assert!(vigilante.set_night_selection_single(townie));

    game.next_phase();

    assert!(puppeteer.alive());
    assert!(!townie.alive());
    assert!(vigilante.alive());
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

    assert!(townie.set_night_selection_single(arso));
    assert!(arso.set_night_selection_single(arso));
    assert!(sher.set_night_selection_single(townie));

    game.next_phase();

    assert!(arso.alive());
    assert!(!townie.alive());
    assert!(sher.alive());
    assert!(gf.alive());
    assert!(vigi.alive());

    assert_contains!(sher.get_messages_after_last_message(
        ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 1}
    ), ChatMessageVariant::SheriffResult{ suspicious: true });

    game.skip_to(Night, 2);
    
    assert!(arso.set_night_selection_single(townie2));
    assert!(sher.set_night_selection_single(townie2));

    game.next_phase();

    assert_contains!(sher.get_messages_after_last_message(
        ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 2}
    ), ChatMessageVariant::SheriffResult{ suspicious: true });

    game.skip_to(Nomination, 3);

    townie2.vote_for_player(Some(arso));
    gf.vote_for_player(Some(arso));
    vigi.vote_for_player(Some(arso));

    game.skip_to(Judgement, 3);

    gf.set_verdict(mafia_server::game::verdict::Verdict::Guilty);

    game.skip_to(Night, 3);

    assert!(sher.set_night_selection_single(townie2));

    game.next_phase();
    
    assert_contains!(sher.get_messages_after_last_message(
        ChatMessageVariant::PhaseChange{phase: PhaseState::Night, day_number: 3}
    ), ChatMessageVariant::SheriffResult{ suspicious: false });

    
}

#[test]
fn pyrolisk_tags_day_one() {
    kit::scenario!(game in Night 1 where
        pyro: Pyrolisk,
        townie: Detective,
        townie2: Detective,
        townie3: Detective,
        townie4: Detective
    );

    assert!(pyro.set_night_selection_single(townie));
    assert!(townie3.set_night_selection_single(pyro));

    game.next_phase();

    assert!(pyro.alive());
    assert!(townie.alive());
    assert!(townie2.alive());
    assert!(townie3.alive());
    assert!(townie4.alive());
    
    assert!(pyro.get_player_tags().get(&pyro.player_ref()).unwrap().contains(&Tag::MorticianTagged));
    assert!(pyro.get_player_tags().get(&townie.player_ref()).unwrap().contains(&Tag::MorticianTagged));
    assert!(pyro.get_player_tags().get(&townie2.player_ref()).is_none());
    assert!(pyro.get_player_tags().get(&townie3.player_ref()).unwrap().contains(&Tag::MorticianTagged));
    assert!(pyro.get_player_tags().get(&townie4.player_ref()).is_none());

    //vote out townie
    game.skip_to(Nomination, 2);

    townie.vote_for_player(Some(townie3));
    townie2.vote_for_player(Some(townie3));
    pyro.vote_for_player(Some(townie3));

    game.skip_to(Judgement, 2);

    townie.set_verdict(Verdict::Guilty);

    game.skip_to(Dusk, 2);

    assert!(game.graves.len() == 1);
    assert!(game.graves.first().unwrap().player == townie3.player_ref());
    assert!(game.graves.first().unwrap().information == GraveInformation::Obscured);
    assert_contains!(pyro.get_messages_after_night(1), ChatMessageVariant::PlayerRoleAndAlibi{
        player: townie3.player_ref(),
        role: Role::Detective,
        will: "".to_string(),
    });
} 

#[test]
fn bodyguard_gets_single_target_jailed_message() {
    kit::scenario!(game in Dusk 2 where
        bg: Bodyguard,
        jailor: Jailor,
        _maf: Mafioso,
        townie: Detective
    );

    jailor.day_target(townie);

    game.next_phase();

    bg.set_night_selection_single(townie);

    game.next_phase();

    assert_eq!(
        bg.get_messages_after_last_message(
            ChatMessageVariant::PhaseChange { 
                phase: PhaseState::Night, day_number: 2
            }
        ),
        vec![
            ChatMessageVariant::Wardblocked,
            /* They should not get a second Wardblocked message */
            ChatMessageVariant::PhaseChange { 
                phase: PhaseState::Obituary, day_number: 3
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
        player3: Mortician,
        player4: Detective
    );

    assert_contains!(
        player1.get_messages(),
        ChatMessageVariant::MartyrRevealed { martyr: martyr.index() }
    );

    martyr.set_night_selection_single(martyr);

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

    martyr.set_night_selection_single(martyr);
    hypnotist.set_night_selection_single(martyr);

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
    kit::scenario!(game in Night 2 where
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

    martyr.set_night_selection_single(martyr);
    doctor.set_night_selection_single(martyr);

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
fn deputy_shoots_townie(){
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
    kit::scenario!(game in Night 2 where
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

    transporter.set_night_selection(vec![player1, player2]);
    gf.set_night_selection_single(ojo);

    game.next_phase();

    assert!(player1.alive());
    assert!(player2.alive());
    assert!(player3.alive());
    assert!(gf.alive());

    assert_contains!(
        ojo.get_messages(), ChatMessageVariant::PlayersRoleRevealed{player: player2.index(), role: Role::Detective}
    );
    assert_contains!(
        ojo.get_messages(), ChatMessageVariant::PlayersRoleRevealed{player: player3.index(), role: Role::Philosopher}
    );
    assert_contains!(
        ojo.get_messages(), ChatMessageVariant::PlayersRoleRevealed{player: gf.index(), role: Role::Godfather}
    );
}

#[test]
/// Sometimes this test fails because of the way tests work
/// if the engineer starts as the apostle and is instantly converted to engineer, then the test might fail
fn apostle_converting_trapped_player_day_later(){
    kit::scenario!(game in Night 2 where
        apostle: Apostle,
        _zealot: Zealot,
        trapped: Detective,
        engineer: Engineer
    );


    assert!(engineer.set_night_selection_single(trapped));

    game.skip_to(Night, 3);

    assert!(apostle.set_night_selection_single(trapped));

    game.next_phase();

    assert_contains!(engineer.get_messages(), ChatMessageVariant::EngineerVisitorsRole { role: Role::Apostle });
    assert!(trapped.role_state().role() != Role::Zealot);
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


    assert!(engineer.set_night_selection_single(trapped));
    assert!(apostle.set_night_selection_single(trapped));

    game.next_phase();

    assert!(trapped.role_state().role() != Role::Zealot);
    assert!(trapped.role_state().role() == Role::Detective);
}

#[test]
fn godfather_dies_to_veteran(){
    kit::scenario!(game in Night 2 where
        vet: Veteran,
        gf: Godfather,
        _maf: Mortician
    );

    assert!(gf.set_night_selection_single(vet));
    assert!(vet.set_night_selection_single(vet));

    game.next_phase();

    assert!(!gf.alive());
    assert!(vet.alive());
}

#[test]
fn minion_leaves_by_winning(){
    kit::scenario!(game in Night 2 where
        t: Veteran,
        gf: Godfather,
        arso: Arsonist,
        min: Minion
    );

    assert!(gf.set_night_selection_single(t));

    game.next_phase();

    assert!(gf.alive());
    assert!(!min.alive());
    assert!(arso.alive());
    assert!(!t.alive());
}
#[test]
fn scarecrow_leaves_by_winning(){
    kit::scenario!(game in Night 2 where
        t: Veteran,
        gf: Godfather,
        arso: Arsonist,
        min: Scarecrow
    );

    assert!(gf.set_night_selection_single(t));

    game.next_phase();

    assert!(gf.alive());
    assert!(!min.alive());
    assert!(arso.alive());
    assert!(!t.alive());
}
#[test]
fn minion_leaves_by_winning_puppeteer(){
    kit::scenario!(game in Night 2 where
        pup: Puppeteer,
        t: Armorsmith,
        t2: Detective,
        gf: Godfather,
        min: Minion
    );

    pup.set_role_state(RoleState::Puppeteer(Puppeteer{
        marionettes_remaining: 3,
        action: PuppeteerAction::String
    }));
    assert!(pup.set_night_selection_single(t));

    game.skip_to(Night, 3);

    assert!(t.set_night_selection_single(t));
    assert!(gf.set_night_selection_single(t2));

    game.next_phase();

    assert!(gf.alive());
    assert!(!min.alive());
    assert!(!t.alive());
    assert!(!t2.alive());
    assert!(pup.alive());
}


#[test]
fn godfather_dies_to_veteran_after_possessed(){
    kit::scenario!(game in Night 2 where
        vet: Veteran,
        gf: Godfather,
        _maf: Mortician,
        min: Minion
    );

    assert!(min.set_night_selection(vec![gf, vet]));
    assert!(vet.set_night_selection_single(vet));

    game.next_phase();

    assert!(!gf.alive());
    assert!(vet.alive());
    assert!(min.alive());
}

#[test]
fn fiends_wildcard_defense_upgrade(){
    kit::scenario!(game in Dusk 2 where
        fiend: FiendsWildcard,
        mafia: MafiaSupportWildcard
    );
    
    fiend.set_role_state(RoleState::FiendsWildcard(FiendsWildcard{
        role: Role::Puppeteer
    }));

    game.next_phase();

    fiend.set_role_state(RoleState::Puppeteer(Puppeteer{
        marionettes_remaining: 3,
        action: PuppeteerAction::String
    }));

    assert!(fiend.role() == Role::Puppeteer);
    assert!(mafia.set_night_selection_single(fiend));
    assert!(fiend.set_night_selection_single(mafia));

    game.next_phase();

    assert!(fiend.alive());
    assert!(mafia.alive());

    assert!(game.game_is_over());
}