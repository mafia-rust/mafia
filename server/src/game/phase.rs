use std::time::Duration;

use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use crate::packet::ToClientPacket;

use super::{
    settings::PhaseTimeSettings,
    Game, player::PlayerReference,
    chat::{ChatGroup, ChatMessage},
    grave::Grave, role::Priority,
};


#[derive(Clone, Copy, PartialEq, Debug, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum PhaseType {
    Morning,
    Discussion,
    Voting,
    Testimony,
    Judgement,
    Evening,
    Night,
}

#[derive(Clone)]
pub enum PhaseState {
    Morning,
    Discussion,
    Voting { trials_left: u8 },
    Testimony { trials_left: u8, player_on_trial: PlayerReference },
    Judgement { trials_left: u8, player_on_trial: PlayerReference },
    Evening { player_on_trial: Option<PlayerReference> },
    Night,
}

pub struct PhaseStateMachine {
    pub time_remaining: Duration,
    pub current_state: PhaseState,
    pub day_number: u8, // Hopefully nobody is having more than 256 days anyway
}

impl PhaseStateMachine {
    pub fn new(times: PhaseTimeSettings) -> Self {
        let current_state = PhaseState::Evening { player_on_trial: None };

        Self {
            time_remaining: times.get_time_for(current_state.phase()),
            day_number: 1,
            current_state,
        }
    }
}

impl PhaseState {
    pub const fn phase(&self) -> PhaseType {
        match self {
            PhaseState::Morning => PhaseType::Morning,
            PhaseState::Discussion => PhaseType::Discussion,
            PhaseState::Voting {..} => PhaseType::Voting,
            PhaseState::Testimony {..} => PhaseType::Testimony,
            PhaseState::Judgement {..} => PhaseType::Judgement,
            PhaseState::Evening {..} => PhaseType::Evening,
            PhaseState::Night => PhaseType::Night,
        }
    }
    
    pub fn start(game: &mut Game) {
        match game.current_phase().clone() {
            PhaseState::Morning => {
                for player_ref in PlayerReference::all_players(game) {
                    if player_ref.night_died(game) {
                        let new_grave = Grave::from_player_night(game, player_ref);
                        player_ref.die(game, new_grave, false);
                    }
                }

                for player_ref in PlayerReference::all_players(game) {
                    if player_ref.night_died(game) {
                        player_ref.invoke_on_any_death(game)
                    }
                }
                

                game.phase_machine.day_number += 1;
            },
            PhaseState::Voting { trials_left } => {
                let required_votes = 1+
                    (PlayerReference::all_players(game).filter(|p| p.alive(game)).count()/2);
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::TrialInformation { required_votes, trials_left });
                

                let packet = ToClientPacket::new_player_votes(game);
                game.send_packet_to_all(packet);
            },
            PhaseState::Testimony { player_on_trial, .. } => {
                game.add_message_to_chat_group(ChatGroup::All, 
                    ChatMessage::PlayerOnTrial { player_index: player_on_trial.index() }
                );
                game.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: player_on_trial.index() });
            },
            PhaseState::Night
            | PhaseState::Discussion
            | PhaseState::Judgement { .. } 
            | PhaseState::Evening { .. } => {}
        }
        game.add_message_to_chat_group(ChatGroup::All, 
            ChatMessage::PhaseChange { 
                phase_type: game.current_phase().phase(), 
                day_number: game.phase_machine.day_number 
            }
        );
    }
    
    /// Returns what phase should come next
    pub fn end(game: &mut Game) -> PhaseState {
        let next = match game.current_phase() {
            PhaseState::Morning => {
                Self::Discussion
            },
            PhaseState::Discussion => {
                Self::Voting { trials_left: 3 }
            },
            PhaseState::Voting {..} => {                
                Self::Night
            },
            &PhaseState::Testimony { trials_left, player_on_trial } => {
                Self::Judgement { trials_left, player_on_trial }
            },
            &PhaseState::Judgement { trials_left, player_on_trial } => {

                game.add_messages_to_chat_group(ChatGroup::All, 
                PlayerReference::all_players(game)
                    .filter(|player_ref|{
                        player_ref.alive(game) && *player_ref != player_on_trial
                    })
                    .map(|player_ref|
                        ChatMessage::JudgementVerdict{
                            voter_player_index: player_ref.index(),
                            verdict: player_ref.verdict(game)
                        }
                    )
                    .collect()
                );
                
                let (guilty, innocent) = game.count_verdict_votes(player_on_trial);
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::TrialVerdict{ 
                    player_on_trial: player_on_trial.index(), 
                    innocent, guilty 
                });
                
                if innocent < guilty {
                    Self::Evening { player_on_trial: Some(player_on_trial) }
                } else if trials_left == 0 {
                    // TODO send no trials left
                    Self::Evening { player_on_trial: None }
                }else{
                    Self::Voting { trials_left }
                }
            },
            &PhaseState::Evening { player_on_trial } => {
                let Some(player_on_trial) = player_on_trial else { return Self::Night };

                let (guilty, innocent) = game.count_verdict_votes(player_on_trial);
                
                if innocent < guilty {
                    let new_grave = Grave::from_player_lynch(game, player_on_trial);
                    player_on_trial.die(game, new_grave, true);
                }

                Self::Night
            },
            PhaseState::Night => {
                for player_ref in PlayerReference::all_players(game){
                    player_ref.set_night_grave_will(game, player_ref.will(game).clone());
                }

                for player_ref in PlayerReference::all_players(game){
                    let visits = player_ref.convert_targets_to_visits(game, player_ref.chosen_targets(game).clone());
                    player_ref.set_night_visits(game, visits.clone());
                }

                for priority in Priority::values(){
                    for player_ref in PlayerReference::all_players(game){
                        player_ref.do_night_action(game, priority);
                    }
                }

                for player_ref in PlayerReference::all_players(game){
                    let mut messages = player_ref.night_messages(game).to_vec();
                    messages.shuffle(&mut rand::thread_rng());
                    messages.sort();
                    player_ref.add_chat_messages(game, messages);
                }

                Self::Morning
            },
        };
        next
    }
    
    pub fn is_day(&self) -> bool {
        self.phase() != PhaseType::Night
    }

    pub fn is_night(&self) -> bool {
        self.phase() == PhaseType::Night
    }
}