use std::collections::HashSet;

use crate::{game::{player::PlayerReference, Game}, vec_set::VecSet};

#[derive(Default)]
pub struct RevealedGroups{
    mafia: RevealedGroup,
    cult: RevealedGroup,
    puppeteer: RevealedGroup
}
impl RevealedGroups{
    pub fn on_remove_role_label(game: &mut Game, player: PlayerReference, concealed_player: PlayerReference){
        InsiderGroupRef::Mafia.on_remove_role_label(game, player, concealed_player);
        InsiderGroupRef::Cult.on_remove_role_label(game, player, concealed_player);
        InsiderGroupRef::Puppeteer.on_remove_role_label(game, player, concealed_player);
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsiderGroupRef{
    Mafia,
    Cult,
    Puppeteer
}
#[derive(Default)]
pub struct RevealedGroup{
    players: VecSet<PlayerReference>
}
impl From<RevealedGroup> for VecSet<PlayerReference> {
    fn from(revealed_group: RevealedGroup)->Self{
        revealed_group.players
    }
}
impl<'a> From<&'a RevealedGroup> for &'a VecSet<PlayerReference> {
    fn from(revealed_group: &'a RevealedGroup)->Self{
        &revealed_group.players
    }
}
impl<'a> From<&'a mut RevealedGroup> for &'a mut VecSet<PlayerReference> {
    fn from(revealed_group: &'a mut RevealedGroup)->Self{
        &mut revealed_group.players
    }
}

impl InsiderGroupRef{
    pub fn all()->HashSet<InsiderGroupRef>{
        vec![
            InsiderGroupRef::Mafia,
            InsiderGroupRef::Cult,
            InsiderGroupRef::Puppeteer
        ].into_iter().collect()
    }
    fn revealed_group<'a>(&self, game: &'a Game)->&'a RevealedGroup{
        match self{
            InsiderGroupRef::Mafia=>&game.revealed_groups.mafia,
            InsiderGroupRef::Cult=>&game.revealed_groups.cult,
            InsiderGroupRef::Puppeteer=>&game.revealed_groups.puppeteer
        }
    }
    fn revealed_group_mut<'a>(&self, game: &'a mut Game)->&'a mut RevealedGroup{
        match self{
            InsiderGroupRef::Mafia=>&mut game.revealed_groups.mafia,
            InsiderGroupRef::Cult=>&mut game.revealed_groups.cult,
            InsiderGroupRef::Puppeteer=>&mut game.revealed_groups.puppeteer
        }
    }
    pub fn reveal_group_players(&self, game: &mut Game){
        let players: VecSet<PlayerReference> = <&RevealedGroup as Into<&VecSet<PlayerReference>>>::
            into(self.revealed_group(game)).clone();

        for a in players.clone() {
            for b in players.clone() {
                a.insert_role_label(game, b);
            }
        }
    }
    pub fn players<'a>(&self, game: &'a Game)->&'a VecSet<PlayerReference>{
        self.revealed_group(game).into()
    }
    pub fn add_player_to_revealed_group(&self, game: &mut Game, player: PlayerReference){
        let players: &mut VecSet<PlayerReference> = self.revealed_group_mut(game).into();
        if players.insert(player).is_none() {
            self.reveal_group_players(game);
        }
    }
    pub fn remove_player_from_revealed_group(&self, game: &mut Game, player: PlayerReference){
        let players: &mut VecSet<PlayerReference> = self.revealed_group_mut(game).into();
        if players.remove(&player).is_some() {
            self.reveal_group_players(game);
        }
    }
    pub fn set_player_revealed_groups(set: VecSet<InsiderGroupRef>, game: &mut Game, player: PlayerReference){
        for group in InsiderGroupRef::all(){
            if set.contains(&group){
                group.add_player_to_revealed_group(game, player);
            }else{
                group.remove_player_from_revealed_group(game, player);
            }
        }
    }
    pub fn start_game_set_player_revealed_groups(set: VecSet<InsiderGroupRef>, game: &mut Game, player: PlayerReference){
        for group in InsiderGroupRef::all(){
            if set.contains(&group){
                let players: &mut VecSet<PlayerReference> = group.revealed_group_mut(game).into();
                players.insert(player);
            }
        }
    }
    pub fn on_remove_role_label(&self, game: &mut Game, player: PlayerReference, concealed_player: PlayerReference){
        
        let players: &VecSet<PlayerReference> = self.revealed_group(game).into();
        if players.contains(&concealed_player) && players.contains(&player) {
            self.reveal_group_players(game);
        }
    }
    pub fn is_player_in_revealed_group(&self, game: &Game, player: PlayerReference)->bool{
        let players: &VecSet<PlayerReference> = self.revealed_group(game).into();
        players.contains(&player)
    }
    pub fn players_both_in_revealed_group(&self, game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        let players: &VecSet<PlayerReference> = self.revealed_group(game).into();
        players.contains(&a) && players.contains(&b)
    }
    pub fn players_in_same_revealed_group(game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        InsiderGroupRef::all().iter().any(|group| group.players_both_in_revealed_group(game, a, b))
    }
}