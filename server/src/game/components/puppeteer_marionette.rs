use std::collections::HashSet;

use crate::game::{
    attack_power::AttackPower, chat::ChatMessageVariant, player::PlayerReference, resolution_state::ResolutionState, role::{
        Priority, Role
    }, tag::Tag, win_condition::WinCondition, Game
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
    pub fn string(game: &mut Game, player: PlayerReference)->bool{
        let mut puppeteer_marionette = game.puppeteer_marionette().clone();

        if player.role(game) == Role::Puppeteer {return false;}
        if !puppeteer_marionette.to_be_converted.insert(player){return false;}

        game.set_puppeteer_marionette(puppeteer_marionette);
        player.set_win_condition(game, WinCondition::ResolutionStateReached { win_if_any: vec![ResolutionState::Fiends].into_iter().collect() });

        for fiend in PuppeteerMarionette::marionettes_and_puppeteer(game){
            fiend.push_night_message(game, ChatMessageVariant::PuppeteerPlayerIsNowMarionette{player: player.index()});
        }

        PuppeteerMarionette::give_tags_and_labels(game);
        true
    }
    pub fn poison(game: &mut Game, player: PlayerReference){
        let mut p = game.puppeteer_marionette().clone();
        player.push_night_message(game, ChatMessageVariant::PuppeteerYouArePoisoned);
        if p.poisoned.insert(player){
            game.set_puppeteer_marionette(p);
        }
    }

    pub fn kill_marionettes(game: &mut Game){
        let marionettes = 
            game.puppeteer_marionette()
                .to_be_converted
                .iter()
                .filter(|p|p.alive(game))
                .map(|p|p.clone())
                .collect::<Vec<_>>();

        PuppeteerMarionette::attack_players(game, marionettes, AttackPower::ProtectionPiercing);
    }
    pub fn kill_poisoned(game: &mut Game){
        let mut puppeteer_marionette = game.puppeteer_marionette().clone();

        let poisoned = game.puppeteer_marionette()
            .poisoned
            .iter()
            .filter(|p|p.alive(game))
            .map(|p|p.clone())
            .collect::<Vec<_>>();

        PuppeteerMarionette::attack_players(game, poisoned, AttackPower::ArmorPiercing);

        puppeteer_marionette.poisoned = HashSet::new();

        game.set_puppeteer_marionette(puppeteer_marionette)
    }
    fn attack_players(game: &mut Game, players: Vec<PlayerReference>, attack_power: AttackPower){
        
        let puppeteers: HashSet<_> = PlayerReference::all_players(game)
            .filter(|p|p.role(game)==Role::Puppeteer)
            .map(|p|p.clone())
            .collect();

        for player in players{
            player.try_night_kill(&puppeteers, game, crate::game::grave::GraveKiller::Role(Role::Puppeteer), attack_power, true);
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
        PuppeteerMarionette::give_tags_and_labels(game);
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        if priority == Priority::Kill{
            PuppeteerMarionette::kill_marionettes(game);
            PuppeteerMarionette::kill_poisoned(game);
        }
    }
}