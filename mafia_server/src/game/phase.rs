use std::{time::Duration, io::Seek};

use serde::{Serialize, Deserialize};

use super::{settings::PhaseTimeSettings, Game, player::{Player, PlayerIndex, self}};


#[derive(Clone, Copy, PartialEq, Debug, Eq, Serialize, Deserialize)]
pub enum PhaseType {
    Morning,
    Discussion,
    Voting,
    Testimony,
    Judgement,
    Evening,
    Night,
}

pub struct PhaseStateMachine {
    pub time_remaining: Duration,
    pub current_state: PhaseType,
    pub day_number: u8, // Hopefully nobody is having more than 256 days anyway
}

impl PhaseStateMachine {
    pub fn new(times: PhaseTimeSettings) -> Self {
        let current_state = PhaseType::Evening;

        Self {
            time_remaining: current_state.get_length(&times),
            day_number: 1,
            current_state,
        }
    }
    pub fn tick(&mut self, game: &mut Game, time_passed: Duration){
        self.time_remaining -= time_passed;
        
        if self.time_remaining > Duration::ZERO{
            return;
        }

        //call end
        self.current_state = self.current_state.end(game);
        //fix time
        self.time_remaining += self.current_state.get_length(&game.settings.phase_times);
        //call start
        self.current_state.start(game);
    }
}

impl PhaseType {
    pub const fn get_length(&self, times: &PhaseTimeSettings) -> Duration {
        match self {
            PhaseType::Morning => times.morning,
            PhaseType::Discussion => times.discussion,
            PhaseType::Voting => times.voting,
            PhaseType::Testimony => times.testimony,
            PhaseType::Judgement => times.judgement,
            PhaseType::Evening => times.evening,
            PhaseType::Night => times.night,
        }
    }

    pub fn start(&mut self, game: &mut Game) {
        // Match phase type and do stuff
        match self {
            PhaseType::Morning => {},
            PhaseType::Discussion => {},
            PhaseType::Voting => {},
            PhaseType::Testimony => {},
            PhaseType::Judgement => {},
            PhaseType::Evening => {},
            PhaseType::Night => {},
        }
    }

    ///returns the next phase
    pub fn end(&mut self, game: &mut Game) -> PhaseType {
        // Match phase type and do stuff
        match self {
            PhaseType::Morning => {
                return Self::Discussion;
            },
            PhaseType::Discussion => {
                return Self::Voting;   
            },
            PhaseType::Voting => {
                return Self::Night;
            },
            PhaseType::Testimony => {
                return Self::Judgement;
            },
            PhaseType::Judgement => {
                return Self::Evening;
            },
            PhaseType::Evening => {
                return Self::Night;
            },
            PhaseType::Night => {

                //get visits
                for player_index in 0..game.players.len(){
                    let player = &mut game.players[player_index];

                    let targets: Vec<PlayerIndex> = player.night_variables.chosen_targets.clone();

                    player.get_role().convert_targets_to_visits(player.index, targets, game);
                }

                //Night actions -- main loop
                for priority in 0..12{
                    for player_index in 0..game.players.len(){
                        //impossible panic when getting player
                        game.players[player_index].get_role().do_night_action(player_index as PlayerIndex, priority, game);
                    }
                }

                //queue night messages
                for player in game.players.iter_mut(){
                    player.add_chat_messages(player.night_variables.night_messages.clone());
                }

                return Self::Morning;
            },
        }
    }

    pub fn is_day(&self) -> bool {
        matches!(self, PhaseType::Night)
    }

}