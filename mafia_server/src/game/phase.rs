use std::{time::Duration, io::Seek};

use serde::{Serialize, Deserialize};

use super::{settings::PhaseTimeSettings, Game, player::{Player, PlayerIndex, self}, chat::{ChatGroup, ChatMessage}, game};


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

    pub fn start(game: &mut Game) {
        // Match phase type and do stuff
        match game.phase_machine.current_state {
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
    pub fn end(game: &mut Game) -> PhaseType {
        // Match phase type and do stuff
        match game.phase_machine.current_state {
            PhaseType::Morning => {
                game.add_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Morning, day_number: game.phase_machine.day_number });

                return Self::Discussion;
            },
            PhaseType::Discussion => {
                game.add_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Discussion, day_number: game.phase_machine.day_number });
                return Self::Voting;   
            },
            PhaseType::Voting => {
                game.add_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Voting, day_number: game.phase_machine.day_number });
                return Self::Night;
            },
            PhaseType::Testimony => {
                game.add_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Testimony, day_number: game.phase_machine.day_number });
                //TODO should be impossible for there to be no player on trial
                game.add_chat_group(ChatGroup::All, ChatMessage::PlayerOnTrial { player_index: game.player_on_trial.unwrap() });
                return Self::Judgement;
            },
            PhaseType::Judgement => {
                game.add_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Judgement, day_number: game.phase_machine.day_number });
                return Self::Evening;
            },
            PhaseType::Evening => {
                game.add_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Evening, day_number: game.phase_machine.day_number });
                return Self::Night;
            },
            PhaseType::Night => {
                game.add_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Night, day_number: game.phase_machine.day_number });

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


                game.phase_machine.day_number+=1;
                return Self::Morning;
            },
        }
    }

    pub fn is_day(&self) -> bool {
        matches!(self, PhaseType::Night)
    }

}