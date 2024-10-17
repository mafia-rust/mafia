use std::collections::HashSet;

use crate::game::{player::PlayerReference, Game};

#[derive(Default)]
pub struct RevealedGroups{
    mafia: RevealedGroup,
    cult: RevealedGroup,
    puppeteer: RevealedGroup
}
impl RevealedGroups{
    pub fn on_remove_role_label(game: &mut Game, player: PlayerReference, concealed_player: PlayerReference){
        RevealedGroupID::Mafia.on_remove_role_label(game, player, concealed_player);
        RevealedGroupID::Cult.on_remove_role_label(game, player, concealed_player);
        RevealedGroupID::Puppeteer.on_remove_role_label(game, player, concealed_player);
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevealedGroupID{
    Mafia,
    Cult,
    Puppeteer
}
#[derive(Default)]
pub struct RevealedGroup{
    players: HashSet<PlayerReference>
}
impl From<RevealedGroup> for HashSet<PlayerReference> {
    fn from(revealed_group: RevealedGroup)->Self{
        revealed_group.players
    }
}
impl<'a> From<&'a RevealedGroup> for &'a HashSet<PlayerReference> {
    fn from(revealed_group: &'a RevealedGroup)->Self{
        &revealed_group.players
    }
}
impl<'a> From<&'a mut RevealedGroup> for &'a mut HashSet<PlayerReference> {
    fn from(revealed_group: &'a mut RevealedGroup)->Self{
        &mut revealed_group.players
    }
}

impl RevealedGroupID{
    pub fn all()->HashSet<RevealedGroupID>{
        vec![
            RevealedGroupID::Mafia,
            RevealedGroupID::Cult,
            RevealedGroupID::Puppeteer
        ].into_iter().collect()
    }
    fn revealed_group<'a>(&self, game: &'a Game)->&'a RevealedGroup{
        match self{
            RevealedGroupID::Mafia=>&game.revealed_groups.mafia,
            RevealedGroupID::Cult=>&game.revealed_groups.cult,
            RevealedGroupID::Puppeteer=>&game.revealed_groups.puppeteer
        }
    }
    fn revealed_group_mut<'a>(&self, game: &'a mut Game)->&'a mut RevealedGroup{
        match self{
            RevealedGroupID::Mafia=>&mut game.revealed_groups.mafia,
            RevealedGroupID::Cult=>&mut game.revealed_groups.cult,
            RevealedGroupID::Puppeteer=>&mut game.revealed_groups.puppeteer
        }
    }
    pub fn reveal_group_players(&self, game: &mut Game){
        let players: HashSet<PlayerReference> = <&RevealedGroup as Into<&HashSet<PlayerReference>>>::
            into(self.revealed_group(game)).clone();

        for a in players.clone() {
            for b in players.clone() {
                a.insert_role_label(game, b);
            }
        }
    }
    pub fn players<'a>(&self, game: &'a Game)->&'a HashSet<PlayerReference>{
        self.revealed_group(game).into()
    }
    pub fn add_player_to_revealed_group(&self, game: &mut Game, player: PlayerReference){
        let players: &mut HashSet<PlayerReference> = self.revealed_group_mut(game).into();
        if players.insert(player) {
            self.reveal_group_players(game);
        }
    }
    pub fn remove_player_from_revealed_group(&self, game: &mut Game, player: PlayerReference){
        let players: &mut HashSet<PlayerReference> = self.revealed_group_mut(game).into();
        if players.remove(&player) {
            self.reveal_group_players(game);
        }
    }
    pub fn set_player_revealed_groups(set: HashSet<RevealedGroupID>, game: &mut Game, player: PlayerReference){
        for group in RevealedGroupID::all(){
            if set.contains(&group){
                group.add_player_to_revealed_group(game, player);
            }else{
                group.remove_player_from_revealed_group(game, player);
            }
        }
    }
    pub fn start_game_set_player_revealed_groups(set: HashSet<RevealedGroupID>, game: &mut Game, player: PlayerReference){
        for group in RevealedGroupID::all(){
            if set.contains(&group){
                let players: &mut HashSet<PlayerReference> = group.revealed_group_mut(game).into();
                players.insert(player);
            }
        }
    }
    pub fn on_remove_role_label(&self, game: &mut Game, player: PlayerReference, concealed_player: PlayerReference){
        
        let players: &HashSet<PlayerReference> = self.revealed_group(game).into();
        if players.contains(&concealed_player) && players.contains(&player) {
            self.reveal_group_players(game);
        }
    }
    pub fn is_player_in_revealed_group(&self, game: &Game, player: PlayerReference)->bool{
        let players: &HashSet<PlayerReference> = self.revealed_group(game).into();
        players.contains(&player)
    }
    pub fn players_both_in_revealed_group(&self, game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        let players: &HashSet<PlayerReference> = self.revealed_group(game).into();
        players.contains(&a) && players.contains(&b)
    }
    pub fn players_in_same_revealed_group(game: &Game, a: PlayerReference, b: PlayerReference)->bool{
        RevealedGroupID::all().iter().any(|group| group.players_both_in_revealed_group(game, a, b))
    }
}