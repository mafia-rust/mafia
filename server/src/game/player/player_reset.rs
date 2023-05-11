

use crate::{game::{phase::PhaseType, Game, verdict::Verdict, grave::GraveRole}, network::packet::ToClientPacket};
use super::{PlayerReference};

impl PlayerReference{
    pub fn tick(&self, game: &mut Game){
        self.send_chat_messages(game);
        // self.send_available_buttons();
    }
    pub fn reset_phase_start(&self, game: &mut Game, phase: PhaseType){
        match phase {
            PhaseType::Morning => {},
            PhaseType::Discussion => {},
            PhaseType::Voting => {
                
                self.set_chosen_vote(game, None);
                self.set_verdict(game, Verdict::Abstain);
            },
            PhaseType::Testimony => {},
            PhaseType::Judgement => {},
            PhaseType::Evening => {},
            PhaseType::Night => {

                self.set_night_alive_tonight(game,   *self.alive(game));
                self.set_night_died(game,            false);
                self.set_night_attacked(game,        false);
                self.set_night_roleblocked(game,     false);
                self.set_night_defense(game,         self.role(game).defense());
                self.set_night_suspicious(game,      self.role(game).suspicious());
                self.set_night_disguised_as(game,    None);
                self.set_chosen_targets(game,  vec![]);
                self.set_night_visits(game,          vec![]);
                self.set_night_messages(game,  vec![]);
                self.set_night_grave_role(game,  GraveRole::Role(self.role(game)));
                self.set_night_grave_killers(game,  vec![]);
                self.set_night_grave_will(game,  self.will(game).clone());   //THIS NEEDS TO BE SET RIGHT BEFORE THEY DIE
                self.set_night_grave_death_notes(game,  vec![]);
            }
        }
    }
}


