use mafia_server::game::{role::{RoleState, transporter::Transporter, vigilante::Vigilante, sheriff::Sheriff, escort::Escort}, phase::PhaseState};

mod common;

#[test]
/// For this test, we're seeing if transporter properly swaps.
/// The vigilante will try to kill town1, which *should* end up killing town2.
/// Likewise, the escort will try to roleblock town2, which *should* end up roleblocking town1.
fn transporter_swaps() {
    common::init_test!(game,
        trans @ Transporter,
        vigi @ Vigilante,
        escort @ Escort,
        town1 @ Sheriff,
        town2 @ Sheriff
    );

    game.start_phase(PhaseState::Night);

    trans.set_chosen_targets(game, vec![town1, town2]);
    vigi.set_chosen_targets(game, vec![town1]);
    escort.set_chosen_targets(game, vec![town2]);

    // TODO, this is kind of annoying, and should be automated
    let next_phase = PhaseState::end(game);
    game.start_phase(next_phase);

    // Regular transportation works (vigi)
    assert!(town1.alive(game));
    assert!(!town2.alive(game));
    
    // Reverse transportation works (esc)
    assert!(town1.night_roleblocked(game));
    assert!(!town2.night_roleblocked(game));
}