use crate::{game::{phase::PhaseType, Game, verdict::Verdict, grave::GraveRole}, network::packet::ToClientPacket};

use super::{PlayerReference, Player};




impl Player{
    pub fn tick(&mut self){
        self.send_chat_messages();
        // self.send_available_buttons();
    }
    pub fn reset_phase_start(game: &mut Game, player_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Morning => {},
            PhaseType::Discussion => {},
            PhaseType::Voting => {
                Player::set_chosen_vote(game, player_ref, None);
                Player::set_verdict(game, player_ref, Verdict::Abstain);
            },
            PhaseType::Testimony => {},
            PhaseType::Judgement => {},
            PhaseType::Evening => {},
            PhaseType::Night => {

                let player = player_ref.deref_mut(game);

                player.night_variables.alive_tonight=  *player.alive();
                player.night_variables.died=           false;
                player.night_variables.attacked=       false;
                player.night_variables.roleblocked=    false;
                player.night_variables.defense=        player.role().defense();
                player.night_variables.suspicious=     player.role().suspicious();
                player.night_variables.disguised_as=   None;
                player.night_variables.chosen_targets= vec![];
                player.night_variables.visits=         vec![];
                player.night_variables.messages= vec![];
                player.night_variables.grave_role= GraveRole::Role(player.role());
                player.night_variables.grave_killers= vec![];
                player.night_variables.grave_will= player.will().clone();   //THIS NEEDS TO BE SET RIGHT BEFORE THEY DIE
                player.night_variables.grave_death_notes= vec![];
                
                player_ref.deref(game).send_packet(ToClientPacket::YourTarget { 
                    player_indices: PlayerReference::ref_vec_to_index(player_ref.deref(game).chosen_targets()) 
                });
            }
        }
    }
}


