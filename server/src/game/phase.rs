use std::{time::Duration, io::Seek};

use serde::{Serialize, Deserialize};

use crate::network::packet::{ToClientPacket, YourButtons};

use super::{settings::PhaseTimeSettings, Game, player::{Player, self, PlayerReference}, chat::{ChatGroup, ChatMessage}, game, verdict::Verdict, grave::Grave, role::{Role, RoleData, Priority}, role_list::Faction};


#[derive(Clone, Copy, PartialEq, Debug, Eq, Serialize, Deserialize)]
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
            PhaseType::Morning => {
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Morning, day_number: game.phase_machine.day_number });

                //generate & add graves
                for player_ref in PlayerReference::all_players(game){
                    if *player_ref.night_died(game){
                        let new_grave = Grave::from_player_night(game, player_ref);
                        game.send_packet_to_all(ToClientPacket::AddGrave{grave: new_grave.clone()});
                        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PlayerDied { grave: new_grave });
                    }
                }
            },
            PhaseType::Discussion => {
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Discussion, day_number: game.phase_machine.day_number });
                
            },
            PhaseType::Voting => {
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Voting, day_number: game.phase_machine.day_number });

                let required_votes = 1+
                    (PlayerReference::all_players(game).iter().filter(|p|*p.alive(game)).collect::<Vec<&PlayerReference>>().len()/2);
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::TrialInformation { required_votes, trials_left: game.trials_left });
                

                let packet = ToClientPacket::new_player_votes(game);
                game.send_packet_to_all(packet);
            },
            PhaseType::Testimony => {
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Testimony, day_number: game.phase_machine.day_number });
                
                //TODO should be impossible for there to be no player on trial therefore unwrap
                game.add_message_to_chat_group(ChatGroup::All, 
                    ChatMessage::PlayerOnTrial { player_index: game.player_on_trial.unwrap().index().clone() }
                );
                game.send_packet_to_all(ToClientPacket::PlayerOnTrial { player_index: game.player_on_trial.unwrap().index().clone() });
            },
            PhaseType::Judgement => {
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Judgement, day_number: game.phase_machine.day_number });

            },
            PhaseType::Evening => {
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Evening, day_number: game.phase_machine.day_number });
                
            },
            PhaseType::Night => {
                //ensure mafia can kill
                //search for mafia godfather or mafioso
                let mut main_mafia_killing_exists = false;
                for player_ref in PlayerReference::all_players(game){
                    if player_ref.role(game) == Role::Mafioso { 
                        main_mafia_killing_exists = true;
                        break;
                    }
                }
                //TODO for now just convert the first person we see to mafioso
                //later set an order for roles
                //ambusher should be converted first
                if !main_mafia_killing_exists{
                    for player_ref in PlayerReference::all_players(game){
                        if player_ref.role(game).faction_alignment().faction() == Faction::Mafia{
                            player_ref.set_role(game, RoleData::Mafioso);
                            break;
                        }
                    }
                }

                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PhaseChange { phase_type: PhaseType::Night, day_number: game.phase_machine.day_number });
            },
        }
    }

    ///returns the next phase
    pub fn end(game: &mut Game) -> PhaseType {
        // Match phase type and do stuff
        match game.phase_machine.current_state {
            PhaseType::Morning => {
                Self::Discussion
            },
            PhaseType::Discussion => {
                Self::Voting 
            },
            PhaseType::Voting => {                
                Self::Night
            },
            PhaseType::Testimony => {
                Self::Judgement
            },
            PhaseType::Judgement => {
                
                let mut innocent = 0;   let mut guilty = 0;
                let mut messages = Vec::new();
                for player_ref in PlayerReference::all_players(game){
                    match *player_ref.verdict(game){
                        Verdict::Innocent => innocent += 1,
                        Verdict::Abstain => {},
                        Verdict::Guilty => guilty += 1,
                    }
                    messages.push(ChatMessage::JudgementVerdict { voter_player_index: *player_ref.index(), verdict: player_ref.verdict(game).clone() });
                }
                game.add_messages_to_chat_group(ChatGroup::All, messages);
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::TrialVerdict { player_on_trial: game.player_on_trial.unwrap().index().clone(), innocent, guilty });
                
                Self::Evening
            },
            PhaseType::Evening => {
                Self::Night
            },
            PhaseType::Night => {

                //MAIN NIGHT CODE

                //get visits
                for player_ref in PlayerReference::all_players(game){
                    let role = player_ref.role(game);
                    let visits = role.convert_targets_to_visits(game, player_ref, player_ref.chosen_targets(game).clone());
                    player_ref.set_night_visits(game, visits);

                }

                //Night actions -- main loop
                for priority in Priority::values(){
                    for player_ref in PlayerReference::all_players(game){
                        player_ref.role(game).do_night_action(game, player_ref, priority);
                    }
                }

                //queue night messages
                for player_ref in PlayerReference::all_players(game){
                    player_ref.add_chat_messages(game, player_ref.night_messages(game).clone());
                }


                game.phase_machine.day_number+=1;
                Self::Morning
            },
        }
    }

    pub fn is_day(&self) -> bool {
        return Self::Night != *self;
    }

}