use std::collections::{HashMap, HashSet};

use vec1::Vec1;

use crate::{
    game::{
        chat::{
            ChatGroup, ChatMessage, ChatMessageVariant
        }, event::on_fast_forward::OnFastForward,
        grave::{GraveKiller, GraveRole},
        role::{Role, RoleState},
        tag::Tag,
        verdict::Verdict,
        visit::Visit,
        Game
    }, 
    packet::ToClientPacket, 
};
use super::PlayerReference;


impl PlayerReference{
    pub fn name<'a>(&self, game: &'a Game) -> &'a String {
        &self.deref(game).name
    }
    
    pub fn role(&self, game: &Game) -> Role {
        self.deref(game).role_state.role()
    }
    pub fn role_state<'a>(&self, game: &'a Game) -> &'a RoleState {
        &self.deref(game).role_state
    }
    pub fn set_role_state(&self, game: &mut Game, new_role_data: RoleState){
        self.deref_mut(game).role_state = new_role_data;
        self.send_packet(game, ToClientPacket::YourRoleState { role_state: self.deref(game).role_state.clone() } );
    }

    pub fn alive(&self, game: &Game) -> bool{
        self.deref(game).alive
    }
    pub fn set_alive(&self, game: &mut Game, alive: bool){
        self.deref_mut(game).alive = alive;

        let mut alive_players = vec![];
        for player in PlayerReference::all_players(game){
            alive_players.push(player.deref(game).alive);
        }
        game.send_packet_to_all(ToClientPacket::PlayerAlive { alive: alive_players });
        game.count_votes_and_start_trial();
    }

    pub fn will<'a>(&self, game: &'a Game) -> &'a String {
        &self.deref(game).will
    }
    pub fn set_will(&self, game: &mut Game, will: String){
        self.deref_mut(game).will = will;
        self.send_packet(game, ToClientPacket::YourWill { will: self.deref(game).will.clone() });
    }
    
    pub fn notes<'a>(&self, game: &'a Game) -> &'a String {
        &self.deref(game).notes
    }
    pub fn set_notes(&self, game: &mut Game, notes: String){
        self.deref_mut(game).notes = notes;
        self.send_packet(game, ToClientPacket::YourNotes { notes: self.deref(game).notes.clone() })
    }

    pub fn crossed_out_outlines<'a>(&self, game: &'a Game) -> &'a Vec<u8> {
        &self.deref(game).crossed_out_outlines
    }
    pub fn set_crossed_out_outlines(&self, game: &mut Game, crossed_out_outlines: Vec<u8>){
        self.deref_mut(game).crossed_out_outlines = crossed_out_outlines;
        self.send_packet(game, ToClientPacket::YourCrossedOutOutlines { crossed_out_outlines: self.deref(game).crossed_out_outlines.clone() });
    }
    
    pub fn death_note<'a>(&self, game: &'a Game) -> &'a Option<String> {
        &self.deref(game).death_note
    }
    pub fn set_death_note(&self, game: &mut Game, death_note: Option<String>){
        self.deref_mut(game).death_note = death_note;
        self.send_packet(game, ToClientPacket::YourDeathNote { death_note: self.deref(game).death_note.clone() })
    }
    
    pub fn role_labels<'a>(&self, game: &'a Game) -> &'a HashSet<PlayerReference>{
        &self.deref(game).role_labels
    }  
    pub fn insert_role_label(&self, game: &mut Game, revealed_player: PlayerReference){
        if
            revealed_player != *self &&
            revealed_player.alive(game) &&
            self.deref_mut(game).role_labels.insert(revealed_player)
        {
            self.add_private_chat_message(game, ChatMessageVariant::PlayersRoleRevealed { player: revealed_player.index(), role: revealed_player.role(game) })
        }


        self.send_packet(game, ToClientPacket::YourRoleLabels{
            role_labels: PlayerReference::ref_map_to_index(self.role_label_map(game)) 
        });
    }
    pub fn remove_role_label(&self, game: &mut Game, concealed_player: PlayerReference){
        if self.deref_mut(game).role_labels.remove(&concealed_player) {
            self.add_private_chat_message(game, ChatMessageVariant::PlayersRoleConcealed { player: concealed_player.index() })
        }

        self.send_packet(game, ToClientPacket::YourRoleLabels{
            role_labels: PlayerReference::ref_map_to_index(self.role_label_map(game)) 
        });
    }

    pub fn player_tags<'a>(&self, game: &'a Game) -> &'a HashMap<PlayerReference, Vec1<Tag>>{
        &self.deref(game).player_tags
    }
    pub fn player_has_tag(&self, game: &Game, key: PlayerReference, value: Tag) -> u8{
        if let Some(player_tags) = self.deref(game).player_tags.get(&key){
            player_tags.iter().filter(|t|**t==value).count() as u8
        }else{
            0
        }
    }
    pub fn push_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag){
        if let Some(player_tags) = self.deref_mut(game).player_tags.get_mut(&key){
            player_tags.push(value);
        }else{
            self.deref_mut(game).player_tags.insert(key, vec1::vec1![value]);
        }
        self.send_packet(game, ToClientPacket::YourPlayerTags { player_tags: PlayerReference::ref_map_to_index(self.deref(game).player_tags.clone()) });
    }
    pub fn remove_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag){
        let Some(player_tags) = self.deref_mut(game).player_tags.get_mut(&key) else {return};

        match Vec1::try_from_vec(
            player_tags.clone()
                .into_iter()
                .filter(|t|*t!=value)
                .collect()
        ){
            Ok(new_player_tags) => *player_tags = new_player_tags,
            Err(_) => {
                self.deref_mut(game).player_tags.remove(&key);
            },
        }
        self.send_packet(game, ToClientPacket::YourPlayerTags{
            player_tags: PlayerReference::ref_map_to_index(self.deref(game).player_tags.clone())
        });
    }
    pub fn remove_player_tag_on_all(&self, game: &mut Game, value: Tag){
        for player_ref in PlayerReference::all_players(game){
            self.remove_player_tag(game, player_ref, value)
        }
    }

    pub fn add_private_chat_message(&self, game: &mut Game, message: ChatMessageVariant) {
        let message = ChatMessage::new_private(message);

        self.add_chat_message(game, message.clone());
    }
    pub fn add_private_chat_messages(&self, game: &mut Game, messages: Vec<ChatMessageVariant>){
        for message in messages.into_iter(){
            self.add_private_chat_message(game, message);
        }
    }
    pub fn add_chat_message(&self, game: &mut Game, message: ChatMessage) {
        self.deref_mut(game).chat_messages.push(message.clone());
        self.deref_mut(game).queued_chat_messages.push(message);
    }

    pub fn set_fast_forward_vote(&self, game: &mut Game, fast_forward_vote: bool) {
        self.deref_mut(game).fast_forward_vote = fast_forward_vote;

        self.send_packet(game, ToClientPacket::YourVoteFastForwardPhase { fast_forward: fast_forward_vote });

        if fast_forward_vote && !game.phase_machine.time_remaining.is_zero() && PlayerReference::all_players(game)
            .filter(|p|p.alive(game)&&(p.could_reconnect(game)||p.is_connected(game)))
            .all(|p| p.fast_forward_vote(game))
        {
            OnFastForward::invoke(game);
        }
    }
    pub fn fast_forward_vote(&self, game: &Game) -> bool{
        self.deref(game).fast_forward_vote
    }

    /* 
    Voting
    */

    pub fn chosen_vote(&self, game: &Game) -> Option<PlayerReference>{
        self.deref(game).voting_variables.chosen_vote
    }
    /// Returns true if this player's vote was changed and packet was sent
    pub fn set_chosen_vote(&self, game: &mut Game, chosen_vote: Option<PlayerReference>, send_chat_message: bool) -> bool{

        if chosen_vote == self.deref(game).voting_variables.chosen_vote ||
            !self.deref(game).alive || self.night_silenced(game) {
            self.deref_mut(game).voting_variables.chosen_vote = None;
            self.send_packet(game, ToClientPacket::YourVoting { 
                player_index: None
            });
            return false;
        }
        
        if let Some(chosen_vote) = chosen_vote {
            if chosen_vote == *self || !chosen_vote.deref(game).alive {
                self.deref_mut(game).voting_variables.chosen_vote = None;
                self.send_packet(game, ToClientPacket::YourVoting { 
                    player_index: None
                });
                return false;
            }
        }
        
        self.deref_mut(game).voting_variables.chosen_vote = chosen_vote;
        self.send_packet(game, ToClientPacket::YourVoting { 
            player_index: self.chosen_vote(game).as_ref().map(PlayerReference::index)
        });
        let player_votes_packet = ToClientPacket::new_player_votes(game);
        game.send_packet_to_all(player_votes_packet);
        
        if send_chat_message {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::Voted{
                voter: self.index(), 
                votee: chosen_vote.as_ref().map(PlayerReference::index)
            });
        }
        
        true
    }

    
    pub fn verdict(&self, game: &Game) -> Verdict{
        self.deref(game).voting_variables.verdict
    }
    pub fn set_verdict(&self, game: &mut Game, verdict: Verdict){
        self.send_packet(game, ToClientPacket::YourJudgement { verdict });
        self.deref_mut(game).voting_variables.verdict = verdict;
    }

    /* 
    Night
    */
    
    pub fn night_died(&self, game: &Game) -> bool {
        self.deref(game).night_variables.died
    }
    pub fn set_night_died(&self, game: &mut Game, died: bool){
        self.deref_mut(game).night_variables.died = died;
    }

    pub fn night_attacked(&self, game: &Game) -> bool {
        self.deref(game).night_variables.attacked
    }
    pub fn set_night_attacked(&self, game: &mut Game, attacked: bool){
        self.deref_mut(game).night_variables.attacked = attacked;
    }

    pub fn night_roleblocked(&self, game: &Game) -> bool {
        self.deref(game).night_variables.roleblocked
    }
    pub fn set_night_roleblocked(&self, game: &mut Game, roleblocked: bool){
        self.deref_mut(game).night_variables.roleblocked = roleblocked;
    }

    pub fn night_defense(&self, game: &Game) -> u8 {
        self.deref(game).night_variables.defense
    }
    pub fn set_night_defense(&self, game: &mut Game, defense: u8){
        self.deref_mut(game).night_variables.defense = defense;
    }

    pub fn night_framed(&self, game: &Game) -> bool {
        self.deref(game).night_variables.framed
    }
    pub fn set_night_framed(&self, game: &mut Game, framed: bool){
        self.deref_mut(game).night_variables.framed = framed;
    }

    pub fn night_appeared_visits<'a>(&self, game: &'a Game) -> &'a Option<Vec<Visit>>{
        &self.deref(game).night_variables.appeared_visits
    }
    pub fn set_night_appeared_visits(&self, game: &mut Game, appeared_visits: Option<Vec<Visit>>){
        self.deref_mut(game).night_variables.appeared_visits = appeared_visits;
    }
    
    pub fn selection<'a>(&self, game: &'a Game) -> &'a Vec<PlayerReference>{
        &self.deref(game).night_variables.selection
    }
    ///returns true if all selections were valid
    pub fn set_selection(&self, game: &mut Game, selection: Vec<PlayerReference>)->bool{
        self.deref_mut(game).night_variables.selection = vec![];

        for target_ref in selection {
            if self.can_select(game, target_ref){
                self.deref_mut(game).night_variables.selection.push(target_ref);
            }else{
                return false;
            }
        }

        let packet = ToClientPacket::YourSelection { 
            player_indices: PlayerReference::ref_vec_to_index(
                &self.deref(game).night_variables.selection
            )
        };
        self.send_packet(game, packet);
        true
    }

    pub fn night_visits<'a>(&self, game: &'a Game) -> &'a Vec<Visit>{
        &self.deref(game).night_variables.visits
    }
    pub fn set_night_visits(&self, game: &mut Game, visits: Vec<Visit>){
        self.deref_mut(game).night_variables.visits = visits;
    }

    pub fn night_messages<'a>(&self, game: &'a Game) -> &'a Vec<ChatMessageVariant> {
        &self.deref(game).night_variables.messages
    }
    pub fn push_night_message(&self, game: &mut Game, message: ChatMessageVariant){
        self.deref_mut(game).night_variables.messages.push(message);
    }
    pub fn set_night_messages(&self, game: &mut Game, messages: Vec<ChatMessageVariant>){
        self.deref_mut(game).night_variables.messages = messages;
    }

    pub fn night_grave_role<'a>(&self, game: &'a Game) -> &'a Option<GraveRole> {
        &self.deref(game).night_variables.grave_role
    }
    pub fn set_night_grave_role(&self, game: &mut Game, grave_role: Option<GraveRole>){
        self.deref_mut(game).night_variables.grave_role = grave_role;
    }

    pub fn night_grave_killers<'a>(&self, game: &'a Game) -> &'a Vec<GraveKiller> {
        &self.deref(game).night_variables.grave_killers
    }
    pub fn push_night_grave_killers(&self, game: &mut Game, grave_killer: GraveKiller){
        self.deref_mut(game).night_variables.grave_killers.push(grave_killer);
    }
    pub fn set_night_grave_killers(&self, game: &mut Game, grave_killers: Vec<GraveKiller>){
        self.deref_mut(game).night_variables.grave_killers = grave_killers;
    }

    pub fn night_grave_will<'a>(&self, game: &'a Game) -> &'a String {
        &self.deref(game).night_variables.grave_will
    }
    pub fn set_night_grave_will(&self, game: &mut Game, grave_will: String){
        self.deref_mut(game).night_variables.grave_will = grave_will;
    }

    pub fn night_grave_death_notes<'a>(&self, game: &'a Game) -> &'a Vec<String> {
        &self.deref(game).night_variables.grave_death_notes
    }
    pub fn push_night_grave_death_notes(&self, game: &mut Game, death_note: String){
        self.deref_mut(game).night_variables.grave_death_notes.push(death_note);
    }
    pub fn set_night_grave_death_notes(&self, game: &mut Game, grave_death_notes: Vec<String>){
        self.deref_mut(game).night_variables.grave_death_notes = grave_death_notes;
    }

    pub fn night_jailed(&self, game: &Game) -> bool {
        self.deref(game).night_variables.jailed
    }
    /// Adds chat message saying that they were jailed, and sends packet
    pub fn set_night_jailed(&self, game: &mut Game, jailed: bool){
        if jailed {

            let mut message_sent = false;
            for chat_group in self.get_current_send_chat_groups(game){
                match chat_group {
                    ChatGroup::All | ChatGroup::Jail | ChatGroup::Interview | ChatGroup::Dead => {},
                    ChatGroup::Mafia | ChatGroup::Cult  => {
                        game.add_message_to_chat_group(
                            chat_group,
                            ChatMessageVariant::JailedSomeone { player_index: self.index() }
                        );
                        message_sent = true;
                    },
                }
            }
            if !message_sent {
                self.add_private_chat_message(game,
                    ChatMessageVariant::JailedSomeone { player_index: self.index() }
                );
            }
        }
        self.deref_mut(game).night_variables.jailed = jailed;
    }

    pub fn night_silenced(&self, game: &Game) -> bool {
        self.deref(game).night_variables.silenced
    }
    pub fn set_night_silenced(&self, game: &mut Game, silenced: bool){
        self.deref_mut(game).night_variables.silenced = silenced;
        if silenced {
            self.push_night_message(game, ChatMessageVariant::Silenced);
            self.send_packet(game, ToClientPacket::YourSendChatGroups { send_chat_groups: 
                self.get_current_send_chat_groups(game)
            });
        }
    }
}



