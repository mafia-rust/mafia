use std::collections::HashMap;

use serde::Serialize;

use crate::game::{Game, role::{Role, RoleData}, chat::ChatMessage, verdict::Verdict, visit::Visit};

use super::Player;

pub type PlayerIndex = u8;
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct PlayerReference {
    index: PlayerIndex
}
impl PlayerReference{
    pub fn new(game: &Game, index: PlayerIndex)->Result<PlayerReference, ()>{
        if !((index as usize) < game.players.len()) { return Err(());} 
        Ok(PlayerReference { index })
    }
    pub fn deref<'a>(&self, game: &'a Game)->&'a Player{
        &game.players[self.index as usize]
    }
    pub fn deref_mut<'a>(&self, game: &'a mut Game)->&'a mut Player{
        &mut game.players[self.index as usize]
    }
    pub fn index(&self)->&PlayerIndex{
        &self.index
    }

    pub fn ref_option_to_index(option: &Option<PlayerReference>)->Option<PlayerIndex>{
        if let Some(reference) = option {
            Some(reference.index().clone())
        }else{
            None
        }
    }
    pub fn ref_vec_to_index(ref_vec: &Vec<PlayerReference>)->Vec<PlayerIndex>{
        ref_vec.into_iter().map(|p|p.index().clone()).collect()
    }
    pub fn ref_map_to_index<T>(ref_map: HashMap<PlayerReference, T>)->HashMap<PlayerIndex, T>{
        ref_map.into_iter().map(|(k,v)|{
            (*k.index(), v)
        }).collect()
    }
    
    pub fn index_option_to_ref(game: &Game, index_option: &Option<PlayerIndex>)->Result<Option<PlayerReference>, ()>{
        match index_option{
            Some(index) => {
                match PlayerReference::new(game, *index){
                    Ok(player_ref) => Ok(Option::Some(player_ref)),
                    Err(_) => Err(()),
                }
            },
            None => Ok(None),
        }
    }
    pub fn index_vec_to_ref(game: &Game, index_vec: &Vec<PlayerIndex>)->Result<Vec<PlayerReference>, ()>{
        let mut out = Vec::new();
        for index in index_vec{
            out.push(match Self::new(game, *index){
                Ok(player_ref) => player_ref,
                Err(_) => {
                    return Err(());
                },
            });
        }
        Ok(out)
    }



    pub fn all_players(game: &Game)->Vec<PlayerReference>{
        let mut out = Vec::new();
        for player_index in 0..game.players.len(){
            out.push(PlayerReference::new(game, player_index as PlayerIndex).unwrap()); //TODO, unwrap here
        }
        out
    }
}

impl PlayerReference{
    pub fn name(&self, game: &Game) -> &String {
        self.deref(game).name()
    }
    
    pub fn role(&self, game: &Game) -> Role {
        self.deref(game).role()
    }
    pub fn role_data(&self, game: &Game) -> &RoleData{
        self.deref(game).role_data()
    }
    pub fn set_role_data(&self, game: &mut Game, new_role_data: RoleData){
        self.deref_mut(game).set_role_data(new_role_data);
    }

    pub fn alive(&self, game: &Game)->&bool{
        &self.deref(game).alive()
    }
    pub fn set_alive(&self, game: &mut Game, alive: bool){
        Player::set_alive(game, *self, alive);
    }

    pub fn will(&self, game: &Game)->&String{
        self.deref(game).will()
    }
    pub fn set_will(&self, game: &mut Game, will: String){
        self.deref_mut(game).set_will(will);
    }
    
    pub fn notes(&self, game: &Game)->&String{
        self.deref(game).notes()
    }
    pub fn set_notes(&self, game: &mut Game, notes: String){
        self.deref_mut(game).notes();
    }
     
    pub fn role_labels(&self, game: &Game)->&HashMap<PlayerReference, Role>{
        self.deref(game).role_labels()
    }  
    pub fn insert_role_label(&self, game: &mut Game, key: PlayerReference, value: Role){
        self.deref_mut(game).insert_role_label(key, value)
    }

    pub fn add_chat_message(&self, game: &mut Game, message: ChatMessage) {
        self.deref_mut(game).add_chat_message(message)
    }
    pub fn add_chat_messages(&self, game: &mut Game, messages: Vec<ChatMessage>){
        self.deref_mut(game).add_chat_messages(messages)
    }

    //VOTING
    pub fn chosen_vote(&self, game: &Game)->&Option<PlayerReference>{
        self.deref(game).chosen_vote()
    }
    /// returns true if players vote was changed
    /// ### checks
    /// Phase == Voting
    /// chosen_vote player exists if its voting a player
    pub fn set_chosen_vote(&self, game: &mut Game, chosen_vote: Option<PlayerReference>)->bool{
        Player::set_chosen_vote(game, *self, chosen_vote)
    }

    
    pub fn verdict(&self, game: &Game)->&Verdict{
        self.deref(game).verdict()
    }
    pub fn set_verdict(&self, game: &mut Game, verdict: Verdict)->bool{
        Player::set_verdict(game, *self, verdict)
    }

    //NIGHT
    pub fn night_alive_tonight(&self, game: &Game)->&bool{
        self.deref(game).night_alive_tonight()
    }
    pub fn set_night_alive_tonight(&self, game: &mut Game, alive_tonight: bool){
        self.deref_mut(game).set_night_alive_tonight(alive_tonight)
    }
    
    pub fn night_died(&self, game: &Game)->&bool{
        self.deref(game).night_died()
    }
    pub fn set_night_died(&self, game: &mut Game, died: bool){
        self.deref_mut(game).set_night_died(died)
    }

    pub fn night_attacked(&self, game: &Game)->&bool{
        self.deref(game).night_attacked()
    }
    pub fn set_night_attacked(&self, game: &mut Game, attacked: bool){
        self.deref_mut(game).set_night_attacked(attacked)
    }

    pub fn night_roleblocked(&self, game: &Game)->&bool{
        self.deref(game).night_roleblocked()
    }
    pub fn set_night_roleblocked(&self, game: &mut Game, roleblocked: bool){
        self.deref_mut(game).set_night_roleblocked(roleblocked)
    }

    pub fn night_defense(&self, game: &Game)->&u8{
        self.deref(game).night_defense()
    }
    pub fn set_night_defense(&self, game: &mut Game, defense: u8){
        self.deref_mut(game).set_night_defense(defense)
    }

    pub fn night_suspicious(&self, game: &Game)->&bool{
        self.deref(game).night_suspicious()
    }
    pub fn set_night_suspicious(&self, game: &mut Game, suspicious: bool){
        self.deref_mut(game).set_night_suspicious(suspicious)
    }

    pub fn night_disguised_as(&self, game: &Game)->&Option<PlayerReference>{
        self.deref(game).night_disguised_as()
    }
    pub fn set_night_disguised_as(&self, game: &mut Game, disguised_as: Option<PlayerReference>){
        self.deref_mut(game).set_night_disguised_as(disguised_as)
    }
    
    pub fn chosen_targets(&self, game: &Game)->&Vec<PlayerReference>{
        self.deref(game).chosen_targets()
    }
    pub fn set_chosen_targets(&self, game: &mut Game, chosen_targets: Vec<PlayerReference>){
        self.deref_mut(game).set_chosen_targets(chosen_targets)
    }

    pub fn night_visits(&self, game: &Game)->&Vec<Visit>{
        self.deref(game).night_visits()
    }
    pub fn set_night_visits(&self, game: &mut Game, visits: Vec<Visit>){
        self.deref_mut(game).set_night_visits(visits)
    }

    pub fn night_messages(&self, game: &Game)->&Vec<ChatMessage>{
        self.deref(game).night_messages()
    }
    pub fn push_night_messages(&self, game: &mut Game, message: ChatMessage){
        self.deref_mut(game).push_night_messages(message)
    }
    pub fn set_night_messages(&self, game: &mut Game, messages: Vec<ChatMessage>){
        self.night_variables.messages = messages;
    }

    pub fn night_grave_role(&self, game: &Game)->&GraveRole{
        &self.night_variables.grave_role
    }
    pub fn set_night_grave_role(&self, game: &mut Game, grave_role: GraveRole){
        self.night_variables.grave_role = grave_role;
    }

    pub fn night_grave_killers(&self, game: &Game)->&Vec<GraveKiller>{
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