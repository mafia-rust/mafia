use std::collections::HashSet;

use crate::game::{
    chat::ChatMessageVariant, player::PlayerReference, role::{
        marionette::Marionette, puppeteer::Puppeteer, Priority, Role, RoleState
    }, tag::Tag, Game
};

impl Game{
    pub fn puppeteer_marionette<'a>(&'a self)->&'a PuppeteerMarionette{
        &self.puppeteer_marionette
    }
    pub fn set_puppeteer_marionette(&mut self, puppeteer_marionette: PuppeteerMarionette){
        self.puppeteer_marionette = puppeteer_marionette;
    }
}

#[derive(Default, Clone)]
pub struct PuppeteerMarionette{
    to_be_converted: HashSet<PlayerReference>,
    poisoned: HashSet<PlayerReference>,
}
impl PuppeteerMarionette{
    pub fn string(game: &mut Game, player: PlayerReference){
        let mut puppeteer_marionette = game.puppeteer_marionette().clone();

        if player.role(game) == Role::Puppeteer {return;}
        if !puppeteer_marionette.to_be_converted.insert(player){return;}

        game.set_puppeteer_marionette(puppeteer_marionette);

        for fiend in PuppeteerMarionette::marionettes_and_puppeteer(game){
            fiend.push_night_message(game, ChatMessageVariant::PuppeteerPlayerIsNowMarionette{player: player.index()});
        }

        PuppeteerMarionette::give_tags_and_labels(game);
    }
    pub fn poison(game: &mut Game, player: PlayerReference){
        let mut p = game.puppeteer_marionette().clone();
        if p.poisoned.insert(player){
            game.set_puppeteer_marionette(p);
            player.push_night_message(game, ChatMessageVariant::PuppeteerYouArePoisoned);
        }
    }

    pub fn kill_marionettes(game: &mut Game){
        let marionettes = game.puppeteer_marionette().to_be_converted.iter().filter(|p|p.alive(game)).map(|p|p.clone()).collect::<Vec<_>>();
        PuppeteerMarionette::attack_players(game, marionettes);
    }
    pub fn kill_poisoned(game: &mut Game){
        let mut puppeteer_marionette = game.puppeteer_marionette().clone();
        let poisoned = game.puppeteer_marionette().poisoned.iter().filter(|p|p.alive(game)).map(|p|p.clone()).collect::<Vec<_>>();
        PuppeteerMarionette::attack_players(game, poisoned);
        puppeteer_marionette.poisoned = HashSet::new();
        game.set_puppeteer_marionette(puppeteer_marionette)
    }
    fn attack_players(game: &mut Game, players: Vec<PlayerReference>){
        for player in players{
            let puppeteers: Vec<_> = PlayerReference::all_players(game)
                .filter(|p|p.role(game)==Role::Puppeteer)
                .map(|p|p.clone())
                .collect();

            if puppeteers.len() == 0 {
                player.try_night_kill_anonymous(game, crate::game::grave::GraveKiller::Role(Role::Puppeteer), 2);
            }
            for puppeteer in puppeteers{
                player.try_night_kill(puppeteer, game, crate::game::grave::GraveKiller::Role(Role::Puppeteer), 2, true);
            }
        }
    }

    pub fn give_tags_and_labels(game: &mut Game){
        let marionettes_and_puppeteer = PuppeteerMarionette::marionettes_and_puppeteer(game);

        for player_a in marionettes_and_puppeteer.clone() {
            for player_b in marionettes_and_puppeteer.clone() {

                player_a.insert_role_label(game, player_b);
                
                if 
                    player_a.player_has_tag(game, player_b, Tag::PuppeteerMarionette) == 0 &&
                    player_b.role(game) != Role::Puppeteer
                {
                    player_a.push_player_tag(game, player_b, Tag::PuppeteerMarionette);
                }
            }
        }
    }

    pub fn has_suspicious_aura_marionette(game: &Game, player: PlayerReference)->bool{
        game.puppeteer_marionette().to_be_converted.contains(&player)
    }
    pub fn marionettes(game: &Game)->HashSet<PlayerReference>{
        game.puppeteer_marionette().clone().to_be_converted
    }
    pub fn puppeteers(game: &Game)->HashSet<PlayerReference>{
        PlayerReference::all_players(game)
            .filter(|p|p.role(game)==Role::Puppeteer)
            .map(|p|p.clone())
            .collect()
    }
    pub fn marionettes_and_puppeteer(game: &Game)->HashSet<PlayerReference>{
        let mut marionettes_and_puppeteer = PuppeteerMarionette::marionettes(game);
        marionettes_and_puppeteer.extend(PuppeteerMarionette::puppeteers(game));
        marionettes_and_puppeteer
    }



    //event listeners

    pub fn on_game_start(game: &mut Game){
        if 
            PlayerReference::all_players(game)
                .any(|p|p.role(game)==Role::Marionette) &&
            !PlayerReference::all_players(game)
                .any(|p|p.role(game)==Role::Puppeteer)
        {
            let marionettes = PlayerReference::all_players(game)
                .filter(|p|p.role(game)==Role::Marionette)
                .filter(|p|p.alive(game))
                .map(|p|p.clone())
                .collect::<Vec<_>>();

            for marionette in marionettes{
                marionette.set_role(game, RoleState::Puppeteer(Puppeteer::default()));
            }
        }
    }
    pub fn on_game_ending(game: &mut Game){
        let mut puppeteer_marionette: PuppeteerMarionette = game.puppeteer_marionette().clone();

        for marionette in puppeteer_marionette.to_be_converted.clone(){
            marionette.set_role(game, RoleState::Marionette(Marionette::default()));
            puppeteer_marionette.to_be_converted.remove(&marionette);
        }
        game.set_puppeteer_marionette(puppeteer_marionette);
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        if priority == Priority::Kill{
            PuppeteerMarionette::kill_marionettes(game);
            PuppeteerMarionette::kill_poisoned(game);
        }
    }
}