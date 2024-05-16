use std::collections::HashSet;

use crate::game::{chat::ChatMessageVariant, player::PlayerReference, role::{marionette::Marionette, Priority, Role, RoleState}, Game};

#[derive(Default, Clone)]
pub struct PuppeteerMarionette{
    to_be_zombified: HashSet<PlayerReference>,
    poisoned: HashSet<PlayerReference>,
}
impl PuppeteerMarionette{
    pub fn zombify(game: &mut Game, player: PlayerReference){
        let mut puppeteer_marionette = game.puppeteer_marionette().clone();
        if puppeteer_marionette.to_be_zombified.insert(player){
            game.set_puppeteer_marionette(puppeteer_marionette);
            player.push_night_message(game, ChatMessageVariant::PuppeteerPlayerIsNowMarionette{player: player.index()});
        }
    }
    pub fn kill_marionettes(game: &mut Game){
        for marionette in game.puppeteer_marionette().to_be_zombified.clone() {
            for puppeteer in PlayerReference::all_players(game){
                if marionette.alive(game) && puppeteer.role(game) == Role::Puppeteer{
                    marionette.try_night_kill(puppeteer, game, crate::game::grave::GraveKiller::Role(Role::Puppeteer), 2, true);
                }
            }
        }
    }
    pub fn kill_poisoned(game: &mut Game){
        let mut puppeteer_marionette = game.puppeteer_marionette().clone();
        for poisoned in puppeteer_marionette.poisoned {
            for puppeteer in PlayerReference::all_players(game){
                if poisoned.alive(game) && puppeteer.role(game) == Role::Puppeteer{
                    poisoned.try_night_kill(puppeteer, game, crate::game::grave::GraveKiller::Role(Role::Puppeteer), 2, true);
                }
            }
        }
        puppeteer_marionette.poisoned = HashSet::new();
        game.set_puppeteer_marionette(puppeteer_marionette)
    }

    pub fn poison(game: &mut Game, player: PlayerReference){
        let mut p = game.puppeteer_marionette().clone();
        if p.poisoned.insert(player){
            game.set_puppeteer_marionette(p);
            player.push_night_message(game, ChatMessageVariant::PuppeteerYouArePoisoned);
        }
    }


    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference){
        let mut puppeteer_marionette: PuppeteerMarionette = game.puppeteer_marionette().clone();

        if puppeteer_marionette.to_be_zombified.contains(&dead_player) {
            dead_player.set_role(game, RoleState::Marionette(Marionette::default()));
            puppeteer_marionette.to_be_zombified.remove(&dead_player);
            game.set_puppeteer_marionette(puppeteer_marionette);
        }
    }
    pub fn on_game_ending(game: &mut Game){
        let mut puppeteer_marionette: PuppeteerMarionette = game.puppeteer_marionette().clone();

        for marionette in puppeteer_marionette.to_be_zombified.clone(){
            if marionette.role(game) == Role::Marionette {continue;}
            marionette.set_role(game, RoleState::Marionette(Marionette::default()));
            puppeteer_marionette.to_be_zombified.remove(&marionette);
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

impl Game{
    pub fn puppeteer_marionette<'a>(&'a self)->&'a PuppeteerMarionette{
        &self.puppeteer_marionette
    }
    pub fn set_puppeteer_marionette(&mut self, puppeteer_marionette: PuppeteerMarionette){
        self.puppeteer_marionette = puppeteer_marionette;
    }
}