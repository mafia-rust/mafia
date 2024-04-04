

use std::time::Duration;

use crate::{client_connection::ClientConnection, game::{grave::GraveKiller, phase::PhaseType, verdict::Verdict, Game}};
use super::PlayerReference;


impl PlayerReference{
    pub fn tick(&self, game: &mut Game, time_passed: Duration){
        match &self.deref(game).connection {
            ClientConnection::Connected(_) => self.send_repeating_data(game),
            ClientConnection::CouldReconnect { disconnect_timer } => {
                match disconnect_timer.saturating_sub(time_passed) {
                    Duration::ZERO => {
                        self.quit(game);
                    },
                    time_remaining => {
                        self.deref_mut(game).connection = ClientConnection::CouldReconnect { disconnect_timer: time_remaining }
                    }
                }
            },
            _ => {}
        }
    }

    pub fn on_phase_start(&self, game: &mut Game, phase: PhaseType){
        match phase {
            PhaseType::Briefing => {},
            PhaseType::Obituary => {
                self.set_night_jailed(game, false);
            },
            PhaseType::Discussion => {},
            PhaseType::Nomination => {
                self.set_chosen_vote(game, None, false);
                self.set_verdict(game, Verdict::Abstain);
            },
            PhaseType::Testimony => {},
            PhaseType::Judgement => {},
            PhaseType::FinalWords => {},
            PhaseType::Dusk => {},
            PhaseType::Night => {
                self.set_night_died(game, false);
                self.set_night_attacked(game, false);
                self.set_night_roleblocked(game, false);
                self.set_night_defense(game, self.role_state(game).defense(game, *self));
                self.set_night_appeared_visits(game, None);
                self.set_night_framed(game, false);
                self.set_night_silenced(game, false);
                self.set_chosen_targets(game, vec![]);
                self.set_night_visits(game, vec![]);
                self.set_night_messages(game, vec![]);
                
                self.set_night_grave_role(game, None);
                self.set_night_grave_killers(game, vec![]);
                self.set_night_grave_will(game, self.will(game).clone());
                self.set_night_grave_death_notes(game, vec![]);

                if self.is_disconnected(game) && self.alive(game){
                    self.set_night_died(game, true);
                    
                    self.set_night_grave_killers(game, vec![GraveKiller::Quit]);
                }
            }
        }

        self.set_fast_forward_vote(game, false);
        self.role_state(game).clone().on_phase_start(game, *self, phase)
    }
}


