use std::collections::HashMap;

use crate::{game::{role::{RoleData, Role}, Game, phase::PhaseType, verdict::Verdict, chat::{ChatGroup, ChatMessage}}, network::packet::ToClientPacket};
use super::{Player, PlayerIndex, player_voting_variables::PlayerVotingVariables, PlayerReference};




impl Player{
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn index(&self) -> &PlayerIndex{
        &self.index
    }
    
    pub fn role(&self) -> Role {
        self.role_data.role()
    }
    pub fn role_data(&self) -> &RoleData{
        &self.role_data
    }
    
    pub fn set_role_data(&mut self, new_role_data: RoleData){
        self.role_data = new_role_data;
        self.send_packet(ToClientPacket::YourRole { role: self.role_data });
    }

    pub fn alive(&self)->&bool{
        &self.alive
    }
    pub fn set_alive(game: &mut Game, player_ref: PlayerReference, alive: bool){
        player_ref.deref_mut(game).alive = alive;

        let mut alive_players = vec![];
        for player in game.players.iter(){
            alive_players.push(player.alive().clone());
        }
        game.send_packet_to_all(ToClientPacket::PlayerAlive { alive: alive_players });
    }

    pub fn will(&self)->&String{
        &self.will
    }
    pub fn set_will(&mut self, will: String){
        self.will = will;
        self.send_packet(ToClientPacket::YourWill { will: self.will().clone() });
    }
    pub fn notes(&self)->&String{
        &self.notes
    }
    pub fn set_notes(&mut self, notes: String){
        self.notes = notes;
        self.send_packet(ToClientPacket::YourNotes { notes: self.notes().clone() })
    }
     
    pub fn role_labels(&self)->&HashMap<PlayerIndex, Role>{
        &self.role_labels
    }  
    pub fn insert_role_label(&mut self, key: PlayerIndex, value: Role){
        self.role_labels.insert(key, value);
        self.send_packet(ToClientPacket::YourRoleLabels { role_labels: self.role_labels.clone() });
    }

    pub fn reset_voting_variables(game: &mut Game, player_ref: PlayerReference){
        player_ref.deref_mut(game).voting_variables.reset();
        // game.get_unchecked_mut_player(player_index).voting_variables.reset();
        // Self::set_verdict(game, player_index, 
        //     game.get_unchecked_mut_player(player_index).verdict().clone(),
        // );
        // Self::set_chosen_vote(game, player_index, game.get_unchecked_mut_player(player_index).chosen_vote().clone());
    }

    pub fn chosen_vote(&self)->&Option<PlayerReference>{
        &self.voting_variables.chosen_vote
    }
    
    /// returns true if players vote was changed
    /// ### checks
    /// Phase == Voting
    /// chosen_vote player exists if its voting a player
    pub fn set_chosen_vote(game: &mut Game, player_ref: PlayerReference, chosen_vote: Option<PlayerReference>)->bool{

        let your_voting_packet: Option<PlayerIndex>;

        if game.current_phase() != PhaseType::Voting || !player_ref.deref(game).alive(){
            return false;
        }
        
        if let Some(chosen_vote) = chosen_vote {
            if chosen_vote == player_ref || !chosen_vote.deref(game).alive(){
                return false;
            }

            your_voting_packet = Some(chosen_vote.index().clone());
        }else{
            your_voting_packet = None;
        }
        
        player_ref.deref(game).voting_variables.chosen_vote = chosen_vote;
        player_ref.deref(game).send_packet(ToClientPacket::YourVoting { player_index: your_voting_packet });

        game.add_message_to_chat_group(ChatGroup::All, 
            ChatMessage::Voted { voter: *player_ref.index(), votee: PlayerReference::ref_option_to_index(&chosen_vote) }
        );
        
        true
    }

    
    pub fn verdict(&self)->&Verdict{
        &self.voting_variables.verdict
    }
    pub fn set_verdict(game: &mut Game, player_ref: PlayerReference, verdict: Verdict)->bool{
        if game.current_phase() != PhaseType::Judgement{
            return false;
        }

        let player = player_ref.deref(game);
                
        player.send_packet(ToClientPacket::YourJudgement { verdict: verdict.clone() });
        if *player.verdict() == verdict {
            return false;
        }
        player.voting_variables.verdict = verdict;
        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::JudgementVote { voter_player_index: player.index });

        true
    }

    pub fn chosen_targets(&self)->&Vec<PlayerReference>{
        &self.night_variables.chosen_targets()
    }
    pub fn set_chosen_targets(game: &mut Game, player_ref: PlayerReference, chosen_targets: Vec<PlayerReference>){
        //TODO can target????
        //TODO Send you targeted someone message in correct chat.
        if game.phase_machine.current_state != PhaseType::Night{
            return;
        }

        player_ref.deref_mut(game).night_variables.set_chosen_targets(vec![]);

        let role = player_ref.deref(game).role();

        for target_index in chosen_targets {
            if role.can_night_target(game, player_ref, target_index){

                let mut old_list = player_ref.deref_mut(game).night_variables.chosen_targets().clone();
                old_list.push(target_index);

                player_ref.deref_mut(game).night_variables.set_chosen_targets(old_list);
            }
        }

        let packet = ToClientPacket::YourTarget { 
            player_indices: PlayerReference::ref_vec_to_index(
                &player_ref.deref(game).chosen_targets()
            )
        };
        player_ref.deref(game).send_packet(packet);
    }
}



