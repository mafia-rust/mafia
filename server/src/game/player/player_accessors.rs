use vec1::Vec1;

use crate::{
    game::{
        chat::{
            ChatMessage, ChatMessageVariant
        },
        event::{
            on_convert::OnConvert, on_fast_forward::OnFastForward,
            on_remove_role_label::OnRemoveRoleLabel
        },
        modifiers::{ModifierType, Modifiers}, role::{Role, RoleState},
        tag::Tag, verdict::Verdict, win_condition::WinCondition, Game
    }, 
    packet::ToClientPacket, vec_map::VecMap, vec_set::VecSet, 
};
use super::PlayerReference;


impl PlayerReference{
    pub fn name<'a>(&self, game: &'a Game) -> &'a String {
        &self.deref(game).name
    }
    pub fn set_name(&self, game: &mut Game, new_name: String) {
        self.deref_mut(game).name = new_name;

        game.send_packet_to_all(ToClientPacket::GamePlayers { 
            players: PlayerReference::all_players(game).map(|p| p.name(game)).cloned().collect()
        });
    }
    
    pub fn role(&self, game: &Game) -> Role {
        self.deref(game).role_state.role()
    }
    pub fn role_state<'a>(&self, game: &'a Game) -> &'a RoleState {
        &self.deref(game).role_state
    }
    pub fn set_role_state(&self, game: &mut Game, new_role_data: impl Into<RoleState>){
        self.deref_mut(game).role_state = new_role_data.into();
        self.send_packet(game, ToClientPacket::YourRoleState {
            role_state: self.deref(game).role_state.clone().get_client_role_state(game, *self)
        });
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
        game.count_nomination_and_start_trial(
            !Modifiers::modifier_is_enabled(game, crate::game::modifiers::ModifierType::ScheduledNominations)
        );
    }

    pub fn will<'a>(&self, game: &'a Game) -> &'a String {
        &self.deref(game).will
    }
    pub fn set_will(&self, game: &mut Game, will: String){
        self.deref_mut(game).will = will;
        self.send_packet(game, ToClientPacket::YourWill { will: self.deref(game).will.clone() });
    }
    
    pub fn notes<'a>(&self, game: &'a Game) -> &'a Vec<String> {
        &self.deref(game).notes
    }
    pub fn set_notes(&self, game: &mut Game, notes: Vec<String>){
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
    
    pub fn role_labels<'a>(&self, game: &'a Game) -> &'a VecSet<PlayerReference>{
        &self.deref(game).role_labels
    }  
    pub fn insert_role_label(&self, game: &mut Game, revealed_player: PlayerReference){
        if
            revealed_player != *self &&
            revealed_player.alive(game) &&
            self.deref_mut(game).role_labels.insert(revealed_player).is_none()
        {
            self.add_private_chat_message(game, ChatMessageVariant::PlayersRoleRevealed { player: revealed_player.index(), role: revealed_player.role(game) })
        }


        self.send_packet(game, ToClientPacket::YourRoleLabels{
            role_labels: PlayerReference::ref_vec_map_to_index(self.role_label_map(game)) 
        });
    }
    pub fn remove_role_label(&self, game: &mut Game, concealed_player: PlayerReference){
        if self.deref_mut(game).role_labels.remove(&concealed_player).is_some() {
            self.add_private_chat_message(game, ChatMessageVariant::PlayersRoleConcealed { player: concealed_player.index() })
        }

        self.send_packet(game, ToClientPacket::YourRoleLabels{
            role_labels: PlayerReference::ref_vec_map_to_index(self.role_label_map(game)) 
        });

        OnRemoveRoleLabel::new(*self, concealed_player).invoke(game);
    }

    pub fn player_tags<'a>(&self, game: &'a Game) -> &'a VecMap<PlayerReference, Vec1<Tag>>{
        &self.deref(game).player_tags
    }
    pub fn player_has_tag(&self, game: &Game, key: PlayerReference, value: Tag) -> usize {
        if let Some(player_tags) = self.deref(game).player_tags.get(&key){
            player_tags.iter().filter(|t|**t==value).count()
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
        self.add_private_chat_message(game, ChatMessageVariant::TagAdded { player: key.index(), tag: value });
        self.send_packet(game, ToClientPacket::YourPlayerTags { player_tags: PlayerReference::ref_vec_map_to_index(self.deref(game).player_tags.clone()) });
    }
    pub fn remove_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag){
        let Some(player_tags) = self.deref_mut(game).player_tags.get_mut(&key) else {return};

        let old_tags = player_tags.clone();
        match Vec1::try_from_vec(
            player_tags.clone()
                .into_iter()
                .filter(|t|*t!=value)
                .collect()
        ){
            Ok(new_player_tags) => {
                *player_tags = new_player_tags
            },
            Err(_) => {
                self.deref_mut(game).player_tags.remove(&key);
            },
        }

        if Some(old_tags) != self.deref_mut(game).player_tags.get(&key).cloned() {
            self.add_private_chat_message(game, ChatMessageVariant::TagRemoved { player: key.index(), tag: value });
            
            self.send_packet(game, ToClientPacket::YourPlayerTags{
                player_tags: PlayerReference::ref_vec_map_to_index(self.deref(game).player_tags.clone())
            });
        }

    }
    pub fn remove_player_tag_on_all(&self, game: &mut Game, value: Tag){
        for player_ref in PlayerReference::all_players(game){
            self.remove_player_tag(game, player_ref, value)
        }
    }

    pub fn win_condition<'a>(&self, game: &'a Game) -> &'a WinCondition {
        &self.deref(game).win_condition
    }
    pub fn set_win_condition(&self, game: &mut Game, win_condition: WinCondition){
        let old_win_condition = self.win_condition(game).clone();
        self.deref_mut(game).win_condition = win_condition.clone();

        OnConvert::new(*self, old_win_condition, win_condition).invoke(game)
    }

    pub fn add_private_chat_message(&self, game: &mut Game, message: ChatMessageVariant) {
        let message = ChatMessage::new_private(message);

        self.add_chat_message(game, message.clone());
    }
    pub fn add_private_chat_messages(&self, game: &mut Game, messages: Vec<ChatMessageVariant>){
        for message in messages {
            self.add_private_chat_message(game, message);
        }
    }
    pub fn add_chat_message(&self, game: &mut Game, message: ChatMessage) {
        self.deref_mut(game).chat_messages.push(message.clone());
        self.deref_mut(game).queued_chat_messages.push(message);
    }
    pub fn chat_messages<'a>(&self, game: &'a Game) -> &'a Vec<ChatMessage> {
        &self.deref(game).chat_messages
    }

    pub fn set_fast_forward_vote(&self, game: &mut Game, fast_forward_vote: bool) {
        self.deref_mut(game).fast_forward_vote = fast_forward_vote;

        self.send_packet(game, ToClientPacket::YourVoteFastForwardPhase { fast_forward: fast_forward_vote });

        if fast_forward_vote && !game.phase_machine.time_remaining.is_some_and(|d|d.is_zero()) && PlayerReference::all_players(game)
            .filter(|p|p.alive(game)&&(p.could_reconnect(game)||p.is_connected(game)))
            .all(|p| p.fast_forward_vote(game))
        {
            OnFastForward::invoke(game);
        }
    }
    pub fn fast_forward_vote(&self, game: &Game) -> bool{
        self.deref(game).fast_forward_vote
    }

    pub fn set_forfeit_vote(&self, game: &mut Game, forfeit: bool) {
        self.deref_mut(game).forfeit_vote = forfeit;
    }
    pub fn forfeit_vote(&self, game: &Game) -> bool{
        self.deref(game).forfeit_vote
    }

    /* 
    Voting
    */

    
    pub fn verdict(&self, game: &Game) -> Verdict{
        self.deref(game).voting_variables.verdict
    }
    pub fn set_verdict(&self, game: &mut Game, mut verdict: Verdict){
        if Modifiers::modifier_is_enabled(game, ModifierType::NoAbstaining) && verdict == Verdict::Abstain {
            verdict = Verdict::Innocent;
        }
        self.send_packet(game, ToClientPacket::YourJudgement { verdict });
        self.deref_mut(game).voting_variables.verdict = verdict;
    }
}



