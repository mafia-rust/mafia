use std::time::Duration;

use serde::{Serialize, Deserialize};

use crate::packet::ToClientPacket;

use super::{
    settings::PhaseTimeSettings,
    Game, player::PlayerReference,
    chat::{ChatGroup, ChatMessage},
    verdict::Verdict, grave::Grave, role::{Priority, Role, mafioso::Mafioso}, role_list::Faction,
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
            PhaseState::Evening {..} => PhaseType::Evening,
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
                        player_ref.die(game, new_grave);
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
                            player_ref.set_role(game, super::role::RoleState::Mafioso(Mafioso::default()));
                            break;
                        }
                    }
                }
            },
            PhaseState::Discussion
            | PhaseState::Judgement { .. } 
            | PhaseState::Evening { .. } => {}
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
                    Self::Evening { player_on_trial: Some(player_on_trial) }
                } else if trials_left == 0 {
                    //TODO send no trials left
                    Self::Evening { player_on_trial: None }
                }else{
                    Self::Voting { trials_left }
                }
            },
            &PhaseState::Evening { player_on_trial } => {
                let Some(player_on_trial) = player_on_trial else { return Self::Night };

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
                    player_on_trial.die(game, new_grave);
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
                    let visits = player_ref.convert_targets_to_visits(game, player_ref.chosen_targets(game).clone());
                    player_ref.set_night_visits(game, visits.clone());
                }

                //Night actions -- main loop
                for priority in Priority::values(){
                    for player_ref in PlayerReference::all_players(game){
                        player_ref.do_night_action(game, priority);
                    }
                }

                //queue night messages
                for player_ref in PlayerReference::all_players(game){
                    player_ref.add_chat_messages(game, player_ref.night_messages(game).to_vec());
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
            PhaseState::Evening {..} => times.evening,
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