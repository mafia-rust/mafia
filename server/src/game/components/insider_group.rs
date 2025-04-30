use serde::{Deserialize, Serialize};

use crate::{game::{chat::{ChatGroup, ChatMessageVariant}, event::{on_add_insider::OnAddInsider, on_remove_insider::OnRemoveInsider, Event}, player::PlayerReference, role_list::RoleAssignment, role_outline_reference::RoleOutlineReference, Game}, packet::ToClientPacket, vec_map::VecMap, vec_set::VecSet};

pub struct InsiderGroups{
    mafia: InsiderGroup,
    cult: InsiderGroup,
    puppeteer: InsiderGroup
}
impl InsiderGroups{
    /// # Safety
    /// player_count is correct
    /// assignments contains all players
    pub unsafe fn new(player_count: u8, assignments: &VecMap<PlayerReference, (RoleOutlineReference, RoleAssignment)>)->Self{
        let mut out = Self{
            mafia: InsiderGroup::default(),
            cult: InsiderGroup::default(),
            puppeteer: InsiderGroup::default()
        };
        for player in PlayerReference::all_players_from_count(player_count){
            for group in assignments.get(&player).expect("Unsafe safety").1.insider_groups(){
                out.get_group_mut(group).players.insert(player);
            }
        }
        out
    }
    pub fn on_conceal_role(game: &mut Game, player: PlayerReference, concealed_player: PlayerReference){
        InsiderGroupID::Mafia.on_conceal_role(game, player, concealed_player);
        InsiderGroupID::Cult.on_conceal_role(game, player, concealed_player);
        InsiderGroupID::Puppeteer.on_conceal_role(game, player, concealed_player);
    }
    fn get_group(&self, id: InsiderGroupID)->&InsiderGroup{
        match id {
            InsiderGroupID::Mafia => &self.mafia,
            InsiderGroupID::Cult => &self.cult,
            InsiderGroupID::Puppeteer => &self.puppeteer,
        }
    }
    fn get_group_mut(&mut self, id: InsiderGroupID)->&mut InsiderGroup{
        match id {
            InsiderGroupID::Mafia => &mut self.mafia,
            InsiderGroupID::Cult => &mut self.cult,
            InsiderGroupID::Puppeteer => &mut self.puppeteer,
        }
    }


    
    pub fn invoke_on_add_insider(game: &mut Game){
        for group in InsiderGroupID::all(){
            for &player in group.players(game).clone().iter(){
                OnAddInsider::new(player, group).invoke(game);
                InsiderGroups::send_player_insider_groups_packet(game, player);
            }
        }
    }
    pub fn start_game_set_player_insider_groups(set: VecSet<InsiderGroupID>, game: &mut Game, player: PlayerReference){
        for group in InsiderGroupID::all(){
            if set.contains(&group){
                let players: &mut VecSet<PlayerReference> = group.players_mut(game);
                players.insert(player);
                OnAddInsider::new(player, group).invoke(game);
            }
        }
        InsiderGroups::send_player_insider_groups_packet(game, player);
        
    }

    // packets
    pub fn send_fellow_insiders_packets(game: &Game, player: PlayerReference){
        let fellow_insiders = PlayerReference::all_players(game)
            .filter(|p| InsiderGroupID::in_same_revealed_group(game, *p, player))
            .map(|p| p.index())
            .collect();

        player.send_packet(game, ToClientPacket::YourFellowInsiders{fellow_insiders});
    }
    pub fn send_player_insider_groups_packet(game: &Game, player: PlayerReference){
        player.send_packet(game, ToClientPacket::YourInsiderGroups{
            insider_groups: InsiderGroupID::all_groups_with_player(game, player)
        });
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InsiderGroupID{
    Mafia,
    Cult,
    Puppeteer
}
#[derive(Default)]
pub struct InsiderGroup{
    players: VecSet<PlayerReference>
}

impl InsiderGroupID{
    //const
    pub fn all()->VecSet<InsiderGroupID>{
        vec![
            InsiderGroupID::Mafia,
            InsiderGroupID::Cult,
            InsiderGroupID::Puppeteer
        ].into_iter().collect()
    }
    pub const fn get_insider_chat_group(&self)->ChatGroup{
        match self{
            InsiderGroupID::Mafia=>ChatGroup::Mafia,
            InsiderGroupID::Cult=>ChatGroup::Cult,
            InsiderGroupID::Puppeteer=>ChatGroup::Puppeteer
        }
    }
    pub fn get_insider_group_from_chat_group(chat: &ChatGroup)->Option<InsiderGroupID>{
        for inside in Self::all() {
            if inside.get_insider_chat_group() == *chat {
                return Some(inside)
            }
        }
        None
    }
    fn deref<'a>(&self, game: &'a Game)->&'a InsiderGroup{
        match self{
            InsiderGroupID::Mafia=>&game.insider_groups.mafia,
            InsiderGroupID::Cult=>&game.insider_groups.cult,
            InsiderGroupID::Puppeteer=>&game.insider_groups.puppeteer
        }
    }
    fn deref_mut<'a>(&self, game: &'a mut Game)->&'a mut InsiderGroup{
        match self{
            InsiderGroupID::Mafia=>&mut game.insider_groups.mafia,
            InsiderGroupID::Cult=>&mut game.insider_groups.cult,
            InsiderGroupID::Puppeteer=>&mut game.insider_groups.puppeteer
        }
    }
    pub fn players<'a>(&self, game: &'a Game)->&'a VecSet<PlayerReference>{
        &self.deref(game).players
    }
    fn players_mut<'a>(&self, game: &'a mut Game)->&'a mut VecSet<PlayerReference>{
        &mut self.deref_mut(game).players
    }

    pub fn add_player(&self, game: &mut Game, player: PlayerReference){
        if self.players_mut(game).insert(player).is_none() {
            self.reveal_group_players(game);
        }
        OnAddInsider::new(player, *self).invoke(game);
        InsiderGroups::send_player_insider_groups_packet(game, player);
    }
    pub fn remove_player(&self, game: &mut Game, player: PlayerReference){
        if self.players_mut(game).remove(&player).is_some() {
            self.reveal_group_players(game);
        }
        OnRemoveInsider::new(player, *self).invoke(game);
        InsiderGroups::send_player_insider_groups_packet(game, player);
    }
    pub fn set_player_insider_groups(set: VecSet<InsiderGroupID>, game: &mut Game, player: PlayerReference){
        for group in InsiderGroupID::all(){
            if set.contains(&group){
                group.add_player(game, player);
            }else{
                group.remove_player(game, player);
            }
        }
    }

    // non related mutations
    pub fn reveal_group_players(&self, game: &mut Game){
        let players: VecSet<PlayerReference> = self.players(game).clone();

        for a in players.iter() {
            InsiderGroups::send_fellow_insiders_packets(game, *a);
            for b in players.iter() {
                a.reveal_players_role(game, *b);
            }
        }
    }

    // Events
    pub fn on_conceal_role(&self, game: &mut Game, player: PlayerReference, concealed_player: PlayerReference){
        
        let players: &VecSet<PlayerReference> = self.players(game);
        if players.contains(&concealed_player) && players.contains(&player) {
            self.reveal_group_players(game);
        }
    }


    // Queries
    pub fn in_any_group(game: &Game, player: PlayerReference)->bool{
        InsiderGroupID::all().into_iter().any(|g|g.contains_player(game, player))
    }
    pub fn contains_player(&self, game: &Game, player: PlayerReference)->bool{
        self.players(game).contains(&player)
    }
    pub fn in_same_revealed_group(game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        InsiderGroupID::all().iter().any(|group|
            group.contains_player(game, a) &&
            group.contains_player(game, b)
        )
    }
    pub fn all_groups_with_player(game: &Game, player_ref: PlayerReference)->VecSet<InsiderGroupID>{
        InsiderGroupID::all()
            .into_iter()
            .filter(|group| 
                group.contains_player(game, player_ref)
            ).collect()
    }

    //other
    pub fn send_message_in_available_insider_chat_or_private(
        game: &mut Game,
        player: PlayerReference,
        message: ChatMessageVariant,
        send_private_backup: bool
    ){
        let mut message_sent = false;
        for chat_group in player.get_current_send_chat_groups(game){
            if Self::get_insider_group_from_chat_group(&chat_group).is_none() {continue};
            game.add_message_to_chat_group(chat_group, message.clone());
            message_sent = true;
        }
        if !message_sent && send_private_backup {
            player.add_private_chat_message(game, message);
        }
    }
}