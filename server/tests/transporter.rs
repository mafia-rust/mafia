use mafia_server::game::{role::{RoleState, transporter::Transporter, vigilante::Vigilante, sheriff::Sheriff, escort::Escort, mafioso::Mafioso, bodyguard::Bodyguard}, phase::PhaseState, chat::ChatMessage};

mod common;

#[test]
/// For this test, we're seeing if transporter properly swaps.
/// The vigilante will try to kill town1, which *should* end up killing town2.
/// Likewise, the escort will try to roleblock town2, which *should* end up roleblocking town1.
fn vigilante_escort_transported_townie() {
    common::init_test!(game,
        trans @ Transporter,
        vigi @ Vigilante,
        escort @ Escort,
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
    escort.set_chosen_targets(game, vec![town2]);

    let next_phase = PhaseState::end(game);
    game.start_phase(next_phase);
    //Morning
    
    // Reverse transportation works (esc)
    assert!(town1.night_roleblocked(game));
    assert!(!town2.night_roleblocked(game));

    // Regular transportation works (vigi)
    assert!(town1.alive(game));
    assert!(!town2.alive(game));

    
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
    common::init_test!(game,
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
fn bodyguard_protect_and_sheriff() {
    common::init_test!(game,
        maf @ Mafioso,
        bg @ Bodyguard,
        sher @ Sheriff
    );
    //Evening
    let next_phase = PhaseState::end(game);
    game.start_phase(next_phase);
    //Night

    maf.set_chosen_targets(game, vec![sher]);
    bg.set_chosen_targets(game, vec![sher]);
    sher.set_chosen_targets(game, vec![maf]);
    
    let next_phase = PhaseState::end(game);
    game.start_phase(next_phase);
    //Morning

    assert!(sher.alive(game));
    assert!(!bg.alive(game));
    assert!(!maf.alive(game));

    assert!(sher.deref(game).chat_messages.contains(&ChatMessage::BodyguardProtectedYou));
    assert!(sher.deref(game).chat_messages.contains(&ChatMessage::SheriffResult { suspicious: true }));
}