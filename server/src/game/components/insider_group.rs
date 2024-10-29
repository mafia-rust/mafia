use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{game::{player::PlayerReference, Game}, packet::ToClientPacket, vec_set::VecSet};

#[derive(Default)]
pub struct InsiderGroups{
    mafia: InsiderGroup,
    cult: InsiderGroup,
    puppeteer: InsiderGroup
}
impl InsiderGroups{
    pub fn on_remove_role_label(game: &mut Game, player: PlayerReference, concealed_player: PlayerReference){
        InsiderGroupID::Mafia.on_remove_role_label(game, player, concealed_player);
        InsiderGroupID::Cult.on_remove_role_label(game, player, concealed_player);
        InsiderGroupID::Puppeteer.on_remove_role_label(game, player, concealed_player);
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
impl From<InsiderGroup> for VecSet<PlayerReference> {
    fn from(revealed_group: InsiderGroup)->Self{
        revealed_group.players
    }
}
impl<'a> From<&'a InsiderGroup> for &'a VecSet<PlayerReference> {
    fn from(revealed_group: &'a InsiderGroup)->Self{
        &revealed_group.players
    }
}
impl<'a> From<&'a mut InsiderGroup> for &'a mut VecSet<PlayerReference> {
    fn from(revealed_group: &'a mut InsiderGroup)->Self{
        &mut revealed_group.players
    }
}

impl InsiderGroupID{
    pub fn all()->HashSet<InsiderGroupID>{
        vec![
            InsiderGroupID::Mafia,
            InsiderGroupID::Cult,
            InsiderGroupID::Puppeteer
        ].into_iter().collect()
    }
    fn revealed_group<'a>(&self, game: &'a Game)->&'a InsiderGroup{
        match self{
            InsiderGroupID::Mafia=>&game.revealed_groups.mafia,
            InsiderGroupID::Cult=>&game.revealed_groups.cult,
            InsiderGroupID::Puppeteer=>&game.revealed_groups.puppeteer
        }
    }
    fn revealed_group_mut<'a>(&self, game: &'a mut Game)->&'a mut InsiderGroup{
        match self{
            InsiderGroupID::Mafia=>&mut game.revealed_groups.mafia,
            InsiderGroupID::Cult=>&mut game.revealed_groups.cult,
            InsiderGroupID::Puppeteer=>&mut game.revealed_groups.puppeteer
        }
    }
    pub fn players<'a>(&self, game: &'a Game)->&'a VecSet<PlayerReference>{
        self.revealed_group(game).into()
    }

    // Mutations
    pub fn add_player_to_revealed_group(&self, game: &mut Game, player: PlayerReference){
        let players: &mut VecSet<PlayerReference> = self.revealed_group_mut(game).into();
        if players.insert(player).is_none() {
            self.reveal_group_players(game);
        }
        Self::send_player_insider_groups(game, player);
    }
    pub fn remove_player_from_revealed_group(&self, game: &mut Game, player: PlayerReference){
        let players: &mut VecSet<PlayerReference> = self.revealed_group_mut(game).into();
        if players.remove(&player).is_some() {
            self.reveal_group_players(game);
        }
        Self::send_player_insider_groups(game, player);
    }
    pub fn set_player_revealed_groups(set: VecSet<InsiderGroupID>, game: &mut Game, player: PlayerReference){
        for group in InsiderGroupID::all(){
            if set.contains(&group){
                group.add_player_to_revealed_group(game, player);
            }else{
                group.remove_player_from_revealed_group(game, player);
            }
        }
    }
    pub fn start_game_set_player_revealed_groups(set: VecSet<InsiderGroupID>, game: &mut Game, player: PlayerReference){
        for group in InsiderGroupID::all(){
            if set.contains(&group){
                let players: &mut VecSet<PlayerReference> = group.revealed_group_mut(game).into();
                players.insert(player);
            }
        }
        Self::send_player_insider_groups(game, player);
    }

    // non related mutations
    pub fn send_player_insider_groups(game: &Game, player: PlayerReference){
        let mut groups = VecSet::new();
        for group in InsiderGroupID::all(){
            if group.is_player_in_revealed_group(game, player){
                groups.insert(group);
            }
        }
        player.send_packet(game, ToClientPacket::YourInsiderGroups{insider_groups: groups});
    }
    pub fn reveal_group_players(&self, game: &mut Game){
        let players: VecSet<PlayerReference> = <&InsiderGroup as Into<&VecSet<PlayerReference>>>::
            into(self.revealed_group(game)).clone();

        for a in players.clone() {
            for b in players.clone() {
                a.insert_role_label(game, b);
            }
        }
    }

    // Events
    pub fn on_remove_role_label(&self, game: &mut Game, player: PlayerReference, concealed_player: PlayerReference){
        
        let players: &VecSet<PlayerReference> = self.revealed_group(game).into();
        if players.contains(&concealed_player) && players.contains(&player) {
            self.reveal_group_players(game);
        }
    }


    // Queries
    pub fn is_player_in_revealed_group(&self, game: &Game, player: PlayerReference)->bool{
        let players: &VecSet<PlayerReference> = self.revealed_group(game).into();
        players.contains(&player)
    }
    pub fn players_both_in_revealed_group(&self, game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        let players: &VecSet<PlayerReference> = self.revealed_group(game).into();
        players.contains(&a) && players.contains(&b)
    }
    pub fn in_same_revealed_group(game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        InsiderGroupID::all().iter().any(|group| group.players_both_in_revealed_group(game, a, b))
    }
    pub fn all_players_in_same_revealed_group_with_actor(game: &Game, actor_ref: PlayerReference)->VecSet<PlayerReference>{
        let mut players = VecSet::new();
        for group in InsiderGroupID::all(){
            if group.is_player_in_revealed_group(game, actor_ref){
                players.extend(group.players(game).clone());
            }
        }
        players
    }
}