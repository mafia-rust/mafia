use std::collections::HashMap;

use crate::{game::{role::{RoleData, Role}, Game, phase::PhaseType, verdict::Verdict, chat::{ChatGroup, ChatMessage}, visit::Visit, grave::{GraveRole, GraveKiller}}, network::packet::ToClientPacket};
use super::{Player, PlayerIndex, PlayerReference};


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
        self.send_packet(ToClientPacket::YourRole { role: self.role() });
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
     
    pub fn role_labels(&self)->&HashMap<PlayerReference, Role>{
        &self.role_labels
    }  
    pub fn insert_role_label(&mut self, key: PlayerReference, value: Role){
        self.role_labels.insert(key, value);
        self.send_packet(ToClientPacket::YourRoleLabels { role_labels: PlayerReference::ref_map_to_index(self.role_labels.clone()) });
    }

    pub fn add_chat_message(&mut self, message: ChatMessage) {
        self.chat_messages.push(message.clone());
        self.queued_chat_messages.push(message);
    }
    pub fn add_chat_messages(&mut self, messages: Vec<ChatMessage>){
        for message in messages.into_iter(){
            self.add_chat_message(message);
        }
    }

    //VOTING
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
            if chosen_vote == player_ref || !chosen_vote.deref(game).alive{
                return false;
            }

            your_voting_packet = Some(chosen_vote.index().clone());
        }else{
            your_voting_packet = None;
        }
        
        player_ref.deref_mut(game).voting_variables.chosen_vote = chosen_vote;
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

        let player = player_ref.deref_mut(game);
                
        player.send_packet(ToClientPacket::YourJudgement { verdict: verdict.clone() });
        if player.voting_variables.verdict == verdict {
            return false;
        }
        player.voting_variables.verdict = verdict;
        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::JudgementVote { voter_player_index: *player_ref.index() });

        true
    }

    //NIGHT
    pub fn night_alive_tonight(&self)->&bool{
        &self.night_variables.alive_tonight
    }
    pub fn set_night_alive_tonight(&mut self, alive_tonight: bool){
        self.night_variables.alive_tonight = alive_tonight;
    }
    
    pub fn night_died(&self)->&bool{
        &self.night_variables.died
    }
    pub fn set_night_died(&mut self, died: bool){
        self.night_variables.died = died;
    }

    pub fn night_attacked(&self)->&bool{
        &self.night_variables.attacked
    }
    pub fn set_night_attacked(&mut self, attacked: bool){
        self.night_variables.attacked = attacked;
    }

    pub fn night_roleblocked(&self)->&bool{
        &self.night_variables.roleblocked
    }
    pub fn set_night_roleblocked(&mut self, roleblocked: bool){
        self.night_variables.roleblocked = roleblocked;
    }

    pub fn night_defense(&self)->&u8{
        &self.night_variables.defense
    }
    pub fn set_night_defense(&mut self, defense: u8){
        self.night_variables.defense = defense;
    }

    pub fn night_suspicious(&self)->&bool{
        &self.night_variables.suspicious
    }
    pub fn set_night_suspicious(&mut self, suspicious: bool){
        self.night_variables.suspicious = suspicious;
    }

    pub fn night_disguised_as(&self)->&Option<PlayerReference>{
        &self.night_variables.disguised_as
    }
    pub fn set_night_disguised_as(&mut self, disguised_as: Option<PlayerReference>){
        self.night_variables.disguised_as = disguised_as;
    }
    
    pub fn chosen_targets(&self)->&Vec<PlayerReference>{
        &self.night_variables.chosen_targets
    }
    pub fn set_chosen_targets(game: &mut Game, player_ref: PlayerReference, chosen_targets: Vec<PlayerReference>){
        //TODO can target????
        //TODO Send you targeted someone message in correct chat.
        if game.phase_machine.current_state != PhaseType::Night{
            return;
        }

        player_ref.deref_mut(game).night_variables.chosen_targets = vec![];

        let role = player_ref.deref(game).role();

        for target_ref in chosen_targets {
            if role.can_night_target(game, player_ref, target_ref){

                player_ref.deref_mut(game).night_variables.chosen_targets.push(target_ref);
            }
        }

        let packet = ToClientPacket::YourTarget { 
            player_indices: PlayerReference::ref_vec_to_index(
                &player_ref.deref(game).night_variables.chosen_targets
            )
        };
        player_ref.deref(game).send_packet(packet);
    }

    pub fn night_visits(&self)->&Vec<Visit>{
        &self.night_variables.visits
    }
    pub fn set_night_visits(&mut self, visits: Vec<Visit>){
        self.night_variables.visits = visits;
    }

    pub fn night_messages(&self)->&Vec<ChatMessage>{
        &self.night_variables.messages
    }
    pub fn push_night_messages(&mut self, message: ChatMessage){
        self.night_variables.messages.push(message);
    }
    pub fn set_night_messages(&mut self, messages: Vec<ChatMessage>){
        self.night_variables.messages = messages;
    }

    pub fn night_grave_role(&self)->&GraveRole{
        &self.night_variables.grave_role
    }
    pub fn set_night_grave_role(&mut self, grave_role: GraveRole){
        self.night_variables.grave_role = grave_role;
    }

    pub fn night_grave_killers(&self)->&Vec<GraveKiller>{
        &self.night_variables.grave_killers
    }
    pub fn push_night_grave_killers(&mut self, grave_killer: GraveKiller){
        self.night_variables.grave_killers.push(grave_killer);
    }
    pub fn set_night_grave_killers(&mut self, grave_killers: Vec<GraveKiller>){
        self.night_variables.grave_killers = grave_killers;
    }

    pub fn night_grave_will(&self)->&String{
        &self.night_variables.grave_will
    }
    pub fn set_night_grave_will(&mut self, grave_will: String){
        self.night_variables.grave_will = grave_will;
    }

    pub fn night_grave_death_notes(&self)->&Vec<String>{
        &self.night_variables.grave_death_notes
    }
    pub fn push_night_grave_death_notes(&mut self, death_note: String){
        self.night_variables.grave_death_notes.push(death_note);
    }
    pub fn set_night_grave_death_notes(&mut self, grave_death_notes: Vec<String>){
        self.night_variables.grave_death_notes = grave_death_notes;
    }


}



