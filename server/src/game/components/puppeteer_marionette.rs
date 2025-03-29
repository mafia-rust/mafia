use std::collections::HashSet;

use crate::{game::{
    attack_power::AttackPower, chat::ChatMessageVariant, 
    player::PlayerReference, 
    role::{
        Priority, Role
    }, tag::Tag, Game
}, vec_set::VecSet};

use super::insider_group::InsiderGroupID;

impl Game{
    pub fn puppeteer_marionette(&self)->&PuppeteerMarionette{
        &self.puppeteer_marionette
    }
    pub fn set_puppeteer_marionette(&mut self, puppeteer_marionette: PuppeteerMarionette){
        self.puppeteer_marionette = puppeteer_marionette;
    }
}

#[derive(Default, Clone)]
pub struct PuppeteerMarionette{
    to_be_converted: HashSet<PlayerReference>,
}
impl PuppeteerMarionette{
    pub fn string(game: &mut Game, player: PlayerReference)->bool{
        let mut puppeteer_marionette = game.puppeteer_marionette().clone();

        if player.role(game) == Role::Puppeteer {return false;}
        if !puppeteer_marionette.to_be_converted.insert(player){return false;}

        game.set_puppeteer_marionette(puppeteer_marionette);

        for fiend in PuppeteerMarionette::marionettes_and_puppeteer(game){
            fiend.push_night_message(game, ChatMessageVariant::PuppeteerPlayerIsNowMarionette{player: player.index()});
        }

        PuppeteerMarionette::give_tags_and_labels(game);
        true
    }

    pub fn kill_marionettes(game: &mut Game){
        let marionettes = 
            game.puppeteer_marionette()
                .to_be_converted
                .iter()
                .filter(|p|p.alive(game))
                .copied()
                .collect::<Vec<_>>();

        PuppeteerMarionette::attack_players(game, marionettes, AttackPower::ProtectionPiercing);
    }
    fn attack_players(game: &mut Game, players: Vec<PlayerReference>, attack_power: AttackPower){
        
        let puppeteers: VecSet<_> = PlayerReference::all_players(game)
            .filter(|p|p.role(game)==Role::Puppeteer)
            .collect();

        for player in players{
            player.try_night_kill(&puppeteers, game, crate::game::grave::GraveKiller::Role(Role::Puppeteer), attack_power, true, false);
        }
    }

    pub fn give_tags_and_labels(game: &mut Game){
        for player_a in InsiderGroupID::Puppeteer.players(game).clone() {
            for player_b in PuppeteerMarionette::marionettes(game).clone() {
                if 
                    player_a.player_has_tag(game, player_b, Tag::PuppeteerMarionette) == 0
                {
                    player_a.push_player_tag(game, player_b, Tag::PuppeteerMarionette);
                }
            }
        }
    }

    pub fn is_marionette(game: &Game, player: PlayerReference)->bool{
        game.puppeteer_marionette().to_be_converted.contains(&player)
    }
    pub fn marionettes(game: &Game)->HashSet<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|p|
                game.puppeteer_marionette().to_be_converted.contains(p)
            )
            .collect()
    }
    pub fn puppeteers(game: &Game)->HashSet<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|p|p.role(game)==Role::Puppeteer)
            .collect()
    }
    pub fn marionettes_and_puppeteer(game: &Game)->HashSet<PlayerReference>{
        let mut marionettes_and_puppeteer = PuppeteerMarionette::marionettes(game);
        marionettes_and_puppeteer.extend(PuppeteerMarionette::puppeteers(game));
        marionettes_and_puppeteer
    }



    //event listeners

    pub fn on_game_start(game: &mut Game){
        PuppeteerMarionette::give_tags_and_labels(game);
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        if priority == Priority::Kill{
            PuppeteerMarionette::kill_marionettes(game);
        }
    }
}