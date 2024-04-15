use std::collections::HashSet;

use crate::game::{grave::GraveKiller, player::PlayerReference, role::Role, tag::Tag, Game};

impl Game {
    pub fn arsonist_doused(&self)->&ArsonistDoused{
        &self.arsonist_doused
    }
    pub fn set_arsonist_doused(&mut self, arsonist_doused: ArsonistDoused){
        self.arsonist_doused = arsonist_doused;
    }
}
#[derive(Default, Clone)]
pub struct ArsonistDoused {
    pub doused_players: HashSet<PlayerReference>,
}
impl ArsonistDoused {
    pub fn douse(mut self, game: &mut Game, player: PlayerReference) {
        if player.role(game) == Role::Arsonist {
            return
        }

        self.doused_players.insert(player);

        ArsonistDoused::tag_doused_players_for_arsonists(&self, game);

        game.set_arsonist_doused(self.clone());
    }
    pub fn clean_doused(mut self, game: &mut Game, player: PlayerReference) {
        self.doused_players.remove(&player);

        ArsonistDoused::tag_doused_players_for_arsonists(&self, game);

        game.set_arsonist_doused(self.clone());
    }
    pub fn ignite(&self, game: &mut Game, igniter: PlayerReference) {
        for player in self.doused_players.clone() {
            if player.role(game) == Role::Arsonist {continue;}
            if !player.alive(game) {continue;}
            player.try_night_kill(igniter, game, GraveKiller::Role(Role::Arsonist), 3, true);
        }
    }
    pub fn doused(&self, player: PlayerReference) -> bool {
        self.doused_players.contains(&player)
    }
    pub fn tag_doused_players_for_arsonists(&self, game: &mut Game) {
        for arsonist in PlayerReference::all_players(game){
            if arsonist.role(game) != Role::Arsonist {continue;}

            arsonist.remove_player_tag_on_all(game, Tag::Doused);
            for player in self.doused_players.clone() {
                arsonist.push_player_tag(game, player, Tag::Doused)
            }
        }
    }
}