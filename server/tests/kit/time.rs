use mafia_server::game::{phase::{PhaseState, PhaseType}, Game};

/// Advance the game to the next phase
pub fn advance_phase(game: &mut Game) -> PhaseState {
    let next_phase = PhaseState::end(game);
    game.start_phase(next_phase.clone());
    next_phase
}

/// Advance the game until the specified phase
/// Warning: If the phase doesn't always happen, like Judgement, then this may panic.
pub fn skip_to_phase(game: &mut Game, phase: PhaseType) -> &PhaseState {
    let mut tries_remaining = 30;
    while game.current_phase().get_type() != phase {
        if tries_remaining == 0 {
            panic!("Never got to phase {phase:?}");
        }
        
        advance_phase(game);
        tries_remaining -= 1;
    }

    game.current_phase()
}

/// Advance the game a given number of days
/// Warning: If the game is currently in a phase that doesn't always happen, like Judgement, then this may panic.
pub fn skip_days(game: &mut Game, days: u8) -> &PhaseState {
    for _ in 0..days {
        let phase = game.current_phase().get_type();
        advance_phase(game);
        skip_to_phase(game, phase);
    }
    game.current_phase()
}