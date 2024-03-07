use std::time::Duration;

use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use crate::packet::ToClientPacket;

use super::{
    settings::PhaseTimeSettings,
    Game, player::PlayerReference,
    chat::{ChatGroup, ChatMessageVariant},
    grave::Grave, role::Priority,
};


#[derive(Clone, Copy, PartialEq, Debug, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum PhaseType {
    Briefing,
    Obituary,
    Discussion,
    Nomination,
    Testimony,
    Judgement,
    FinalWords,
    Dusk,
    Night,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PhaseState {
    Briefing,
    Obituary,
    Discussion,
    #[serde(rename_all = "camelCase")]
    Nomination { trials_left: u8 },
    #[serde(rename_all = "camelCase")]
    Testimony { trials_left: u8, player_on_trial: PlayerReference },
    #[serde(rename_all = "camelCase")]
    Judgement { trials_left: u8, player_on_trial: PlayerReference },
    #[serde(rename_all = "camelCase")]
    FinalWords { player_on_trial: PlayerReference },
    Dusk,
    Night,
}

pub struct PhaseStateMachine {
    pub time_remaining: Duration,
    pub current_state: PhaseState,
    pub day_number: u8, // Hopefully nobody is having more than 256 days anyway
}

impl PhaseStateMachine {
    pub fn new(times: PhaseTimeSettings) -> Self {
        let current_state = PhaseState::Briefing;

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
            PhaseState::Briefing => PhaseType::Briefing,
            PhaseState::Obituary => PhaseType::Obituary,
            PhaseState::Discussion => PhaseType::Discussion,
            PhaseState::Nomination {..} => PhaseType::Nomination,
            PhaseState::Testimony {..} => PhaseType::Testimony,
            PhaseState::Judgement {..} => PhaseType::Judgement,
            PhaseState::FinalWords {..} => PhaseType::FinalWords,
            PhaseState::Dusk => PhaseType::Dusk,
            PhaseState::Night => PhaseType::Night,
        }
    }
    
    pub fn start(game: &mut Game) {
        match game.current_phase().clone() {
            PhaseState::Obituary => {
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
            PhaseState::Nomination { trials_left } => {
                let required_votes = 1+
                    (PlayerReference::all_players(game).filter(|p| p.alive(game)).count()/2);
                game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::TrialInformation { required_votes, trials_left });
                

                let packet = ToClientPacket::new_player_votes(game);
                game.send_packet_to_all(packet);
            },
            PhaseState::Testimony { player_on_trial, .. } => {
                game.add_message_to_chat_group(ChatGroup::All, 
                    ChatMessageVariant::PlayerOnTrial { player_index: player_on_trial.index() }
                );
                game.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: player_on_trial.index() });
            },
            PhaseState::Briefing 
            | PhaseState::Night
            | PhaseState::Discussion
            | PhaseState::Judgement { .. } 
            | PhaseState::FinalWords { .. }
            | PhaseState::Dusk => {}
        }

        if PhaseState::Briefing == *game.current_phase() {return;}

        game.add_message_to_chat_group(ChatGroup::All, 
            ChatMessageVariant::PhaseChange { 
                phase: game.current_phase().clone(), 
                day_number: game.phase_machine.day_number 
            }
        );
        
    }
    
    /// Returns what phase should come next
    pub fn end(game: &mut Game) -> PhaseState {
        let next = match game.current_phase() {
            PhaseState::Briefing => {
                Self::Dusk
            },
            PhaseState::Obituary => {
                Self::Discussion
            },
            PhaseState::Discussion => {
                Self::Nomination { trials_left: 3 }
            },
            PhaseState::Nomination {..} => {                
                Self::Dusk
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
                        ChatMessageVariant::JudgementVerdict{
                            voter_player_index: player_ref.index(),
                            verdict: player_ref.verdict(game)
                        }
                    )
                    .collect()
                );
                
                let (guilty, innocent) = game.count_verdict_votes(player_on_trial);
                game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::TrialVerdict{ 
                    player_on_trial: player_on_trial.index(), 
                    innocent, guilty 
                });
                
                if innocent < guilty {
                    Self::FinalWords { player_on_trial }
                } else if trials_left == 0 {
                    Self::Dusk
                }else{
                    Self::Nomination { trials_left }
                }
            },
            &PhaseState::FinalWords { player_on_trial } => {
                let (guilty, innocent) = game.count_verdict_votes(player_on_trial);
                
                if innocent < guilty {
                    let new_grave = Grave::from_player_lynch(game, player_on_trial);
                    player_on_trial.die(game, new_grave, true);
                }

                Self::Dusk
            },
            PhaseState::Dusk => {
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
                    player_ref.add_private_chat_messages(game, messages);
                }

                Self::Obituary
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