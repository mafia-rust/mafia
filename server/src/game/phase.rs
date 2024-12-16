use std::time::Duration;

use serde::{Serialize, Deserialize};

use crate::packet::ToClientPacket;

use super::{
    chat::{ChatGroup, ChatMessageVariant}, event::{before_phase_end::BeforePhaseEnd, on_any_death::OnAnyDeath, on_night_priority::OnNightPriority, on_phase_start::OnPhaseStart}, grave::Grave, player::PlayerReference, role::Priority, settings::PhaseTimeSettings, Game
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
    Nomination { trials_left: u8, nomination_time_remaining: Duration },
    #[serde(rename_all = "camelCase")]
    Testimony { trials_left: u8, player_on_trial: PlayerReference, nomination_time_remaining: Duration },
    #[serde(rename_all = "camelCase")]
    Judgement { trials_left: u8, player_on_trial: PlayerReference, nomination_time_remaining: Duration },
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

    pub fn get_time_remaining(&self) -> Duration {
        self.time_remaining
    }

    pub fn set_time_remaining(&mut self, time: Duration) {
        self.time_remaining = time;
    }

    pub fn next_phase(game: &mut Game, force_next_phase: Option<PhaseState>) {
        BeforePhaseEnd::new(game.current_phase().phase()).invoke(game);
        let mut new_phase = PhaseState::end(game);

        if let Some(forced_new_phase) = force_next_phase {
            new_phase = forced_new_phase;
        }

        game.phase_machine.current_state = new_phase;
        game.phase_machine.time_remaining = PhaseStateMachine::get_start_time(game, game.current_phase().phase());

        PhaseState::start(game);
        OnPhaseStart::new(game.current_phase().phase()).invoke(game);
    }

    pub fn get_start_time(game: &Game, phase: PhaseType) -> Duration {
        let mut time = game.settings.phase_times.get_time_for(phase);
        //if there are less than 3 players alive then the game is sped up by 2x
        if PlayerReference::all_players(game).filter(|p|p.alive(game)).count() <= 3{
            time /= 2;
        }
        time
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

        
        game.add_message_to_chat_group(ChatGroup::All, 
            ChatMessageVariant::PhaseChange { 
                phase: game.current_phase().clone(),
                //need this if statement because the day number should be increased for obituary phase
                day_number: if game.current_phase().phase() != PhaseType::Obituary {game.phase_machine.day_number} else {game.phase_machine.day_number+1}
            }
        );
        
        match game.current_phase().clone() {
            PhaseState::Obituary => {

                for player in PlayerReference::all_players(game){
                    let Some(role_state) = player.night_convert_role_to(game).clone() else {continue};
                    player.set_role(game, role_state);
                }

                let mut events = Vec::<OnAnyDeath>::new();

                for player_ref in PlayerReference::all_players(game) {
                    if player_ref.night_died(game) {
                        let new_grave = Grave::from_player_night(game, player_ref);
                        events.push(player_ref.die_return_event(game, new_grave));
                    }
                }

                events.into_iter().for_each(|f| f.invoke(game));

                game.phase_machine.day_number += 1;
            },
            PhaseState::Nomination { trials_left, nomination_time_remaining } => {
                game.phase_machine.set_time_remaining(nomination_time_remaining);

                let required_votes = game.nomination_votes_required();
                game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::TrialInformation { required_votes, trials_left });
                

                let packet = ToClientPacket::new_player_votes(game);
                game.send_packet_to_all(packet);
            },
            PhaseState::Testimony { player_on_trial, .. } => {
                game.add_message_to_chat_group(ChatGroup::All, 
                    ChatMessageVariant::PlayerNominated {
                        player_index: player_on_trial.index(),
                        players_voted: PlayerReference::all_players(game)
                            .filter(|player_ref| player_ref.chosen_vote(game) == Some(player_on_trial))
                            .map(|player_ref| player_ref.index())
                            .collect()
                    }
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
                Self::Nomination {
                    trials_left: 3,
                    nomination_time_remaining: PhaseStateMachine::get_start_time(game, PhaseType::Nomination)
                }
            },
            PhaseState::Nomination {..} => {                
                Self::Dusk
            },
            &PhaseState::Testimony { trials_left, player_on_trial, nomination_time_remaining } => {
                Self::Judgement { trials_left, player_on_trial, nomination_time_remaining }
            },
            &PhaseState::Judgement { trials_left, player_on_trial, nomination_time_remaining } => {

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
                    Self::Nomination { trials_left, nomination_time_remaining }
                }
            },
            &PhaseState::FinalWords { player_on_trial } => {
                let (guilty, innocent) = game.count_verdict_votes(player_on_trial);
                
                if innocent < guilty {
                    let new_grave = Grave::from_player_lynch(game, player_on_trial);
                    player_on_trial.die(game, new_grave);
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
                    let visits = player_ref.convert_selection_to_visits(game);
                    player_ref.set_night_visits(game, visits.clone());
                }

                for priority in Priority::values(){
                    OnNightPriority::new(priority).invoke(game);
                    for player_ref in PlayerReference::all_players(game){
                        player_ref.do_night_action(game, priority);
                    }
                }

                for player_ref in PlayerReference::all_players(game){
                    player_ref.push_night_messages_to_player(game);
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