use std::time::Duration;

use serde::{Serialize, Deserialize};

use crate::packet::ToClientPacket;

use super::{
    settings::PhaseTimeSettings,
    Game, player::PlayerReference,
    chat::{ChatGroup, ChatMessage, night_message::NightInformation},
    verdict::Verdict, grave::Grave, role::{Role, Priority},
    role_list::Faction
};


#[derive(Clone, Copy, PartialEq, Debug, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PhaseType {
    Morning,
    Discussion,
    Voting,
    Testimony,
    Judgement,
    Evening,
    FinalWords,
    Night,
}

#[derive(Clone)]
pub enum PhaseState {
    Morning,
    Discussion,
    Voting { trials_left: u8 },
    Testimony { trials_left: u8, player_on_trial: PlayerReference },
    Judgement { trials_left: u8, player_on_trial: PlayerReference },
    FinalWords { player_on_trial: PlayerReference },
    Evening,
    Night,
}

pub struct PhaseStateMachine {
    pub time_remaining: Duration,
    pub current_state: PhaseState,
    pub day_number: u8, // Hopefully nobody is having more than 256 days anyway
}

impl PhaseStateMachine {
    pub fn new(times: PhaseTimeSettings) -> Self {
        let current_state = PhaseState::Evening;

        Self {
            time_remaining: current_state.get_length(&times),
            day_number: 1,
            current_state,
        }
    }
}

impl PhaseState {
    pub const fn get_type(&self) -> PhaseType {
        match self {
            PhaseState::Morning => PhaseType::Morning,
            PhaseState::Discussion => PhaseType::Discussion,
            PhaseState::Voting {..} => PhaseType::Voting,
            PhaseState::Testimony {..} => PhaseType::Testimony,
            PhaseState::Judgement {..} => PhaseType::Judgement,
            PhaseState::Evening => PhaseType::Evening,
            PhaseState::FinalWords {..} => PhaseType::FinalWords,
            PhaseState::Night => PhaseType::Night,
        }
    }
    
    pub fn start(game: &mut Game) {
        // Match phase type and do stuff
        game.add_message_to_chat_group(ChatGroup::All, 
            ChatMessage::PhaseChange { 
                phase_type: game.current_phase().get_type(), 
                day_number: game.phase_machine.day_number 
            }
        );
        match game.current_phase().clone() {
            PhaseState::Morning => {
                //generate & add graves
                for player_ref in PlayerReference::all_players(game){
                    if player_ref.night_died(game) {
                        let new_grave = Grave::from_player_night(game, player_ref);
                        game.graves.push(new_grave.clone());
                        game.send_packet_to_all(ToClientPacket::AddGrave{grave: new_grave.clone()});
                        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PlayerDied { grave: new_grave.clone() });
                        if let Some(role) = new_grave.role.get_role(){
                            for other_player_ref in PlayerReference::all_players(game){
                                other_player_ref.insert_role_label(game, player_ref, role);
                            }
                        }
                    }
                }

                game.phase_machine.day_number+=1;   //day_number packet gets sent right after morning starts
            },
            PhaseState::Voting { trials_left } => {
                let required_votes = 1+
                    (PlayerReference::all_players(game).iter().filter(|p| p.alive(game)).collect::<Vec<&PlayerReference>>().len()/2);
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
            PhaseState::Night => {
                //TODO move this potentially
                //ensure mafia can kill
                //search for mafia godfather or mafioso
                let mut main_mafia_killing_exists = false;


                for player_ref in PlayerReference::all_players(game){
                    if player_ref.role(game) == Role::Mafioso && player_ref.alive(game) { 
                        main_mafia_killing_exists = true;
                        break;
                    }
                }

                //TODO for now just convert the first person we see to mafioso
                //later set an order for roles
                //ambusher should be converted first
                if !main_mafia_killing_exists{
                    for player_ref in PlayerReference::all_players(game){
                        if player_ref.role(game).faction_alignment().faction() == Faction::Mafia && player_ref.alive(game){
                            player_ref.set_role(game, Role::Mafioso);
                            break;
                        }
                    }
                }
            },
            PhaseState::Discussion
            | PhaseState::Judgement { .. } 
            | PhaseState::Evening
            | PhaseState::FinalWords { .. } => {}
        }
    }
    
    ///returns the next phase
    pub fn end(game: &mut Game) -> PhaseState {
        // Match phase type and do stuff
        match game.current_phase() {
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
                let mut innocent = 0;
                let mut guilty = 0;
                let mut messages = Vec::new();
                for player_ref in PlayerReference::all_players(game){
                    if !player_ref.alive(game) || player_ref == player_on_trial {
                        continue;
                    }
                    match player_ref.verdict(game){
                        Verdict::Innocent => innocent += 1,
                        Verdict::Abstain => {},
                        Verdict::Guilty => guilty += 1,
                    }
                    messages.push(ChatMessage::JudgementVerdict{
                        voter_player_index: player_ref.index(),
                        verdict: player_ref.verdict(game)
                    });
                }
                game.add_messages_to_chat_group(ChatGroup::All, messages);
                game.add_message_to_chat_group(ChatGroup::All, ChatMessage::TrialVerdict{ 
                        player_on_trial: player_on_trial.index(), 
                        innocent, guilty 
                });
                
                if innocent < guilty {
                    Self::FinalWords { player_on_trial }
                } else if trials_left == 0 {
                    //TODO send no trials left
                    Self::Evening
                }else{
                    Self::Voting { trials_left }
                }
            },
            PhaseState::Evening => {
                Self::Night
            }
            &PhaseState::FinalWords { player_on_trial } => {
                let mut guilty = 0;
                let mut innocent = 0;
                for player_ref in PlayerReference::all_players(game){
                    match player_ref.verdict(game) {
                        Verdict::Innocent => innocent += 1,
                        Verdict::Abstain => {},
                        Verdict::Guilty => guilty += 1,
                    }
                }
                if innocent < guilty {
                    player_on_trial.set_alive(game, false);

                    let new_grave = Grave::from_player_lynch(game, player_on_trial);
                    game.graves.push(new_grave.clone());
                    game.send_packet_to_all(ToClientPacket::AddGrave{grave: new_grave.clone()});
                    game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PlayerDied { grave: new_grave.clone() });
                    if let Some(role) = new_grave.role.get_role(){
                        for other_player_ref in PlayerReference::all_players(game){
                            other_player_ref.insert_role_label(game, player_on_trial, role);
                        }
                    }
                }

                Self::Night
            },
            PhaseState::Night => {

                //MAIN NIGHT CODE

                //get wills
                for player_ref in PlayerReference::all_players(game){
                    player_ref.set_night_grave_will(game, player_ref.will(game).clone());
                }

                //get visits
                for player_ref in PlayerReference::all_players(game){
                    let role_state = player_ref.role_state(game);
                    let visits = role_state.convert_targets_to_visits(game, player_ref, player_ref.chosen_targets(game).clone());
                    player_ref.set_night_visits(game, visits.clone());
                    player_ref.set_night_appeared_visits(game, visits);

                }

                //Night actions -- main loop
                for priority in Priority::values(){
                    for player_ref in PlayerReference::all_players(game){
                        player_ref.role_state(game).do_night_action(game, player_ref, priority);
                    }
                }

                //queue night messages
                for player_ref in PlayerReference::all_players(game){
                    player_ref.add_chat_messages(game, NightInformation::to_chat_message_vec(player_ref.night_messages(game)));
                }


                Self::Morning
            },
        }
    }

    pub const fn get_length(&self, times: &PhaseTimeSettings) -> Duration {
        match self {
            PhaseState::Morning => times.morning,
            PhaseState::Discussion => times.discussion,
            PhaseState::Voting {..} => times.voting,
            PhaseState::Testimony {..} => times.testimony,
            PhaseState::Judgement {..} => times.judgement,
            PhaseState::Evening => times.evening,
            PhaseState::FinalWords {..} => times.final_words,
            PhaseState::Night => times.night,
        }
    }
    
    pub fn is_day(&self) -> bool {
        self.get_type() != PhaseType::Night
    }

    pub fn is_night(&self) -> bool {
        self.get_type() == PhaseType::Night
    }
}