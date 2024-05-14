use std::collections::HashSet;

use crate::game::{chat::ChatMessageVariant, player::PlayerReference, role::{zombie::Zombie, Role, RoleState}, Game};

#[derive(Default, Clone)]
pub struct LichZombie{
    to_be_zombified: HashSet<PlayerReference>
}
impl LichZombie{
    pub fn zombify(game: &mut Game, player: PlayerReference){
        let mut lich_zombie = game.lich_zombie().clone();
        if lich_zombie.to_be_zombified.insert(player){
            game.set_lich_zombie(lich_zombie);
            player.push_night_message(game, ChatMessageVariant::LichPlayerIsNowZombie{player: player.index()});
        }
    }
    pub fn kill_zombies(game: &mut Game){
        for zombie in game.lich_zombie().to_be_zombified.clone() {
            for lich in PlayerReference::all_players(game){
                if zombie.alive(game){
                    zombie.try_night_kill(lich, game, crate::game::grave::GraveKiller::Role(Role::Lich), 2, true);
                }
            }
        }
    }
    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference){
        let mut lich_zombie: LichZombie = game.lich_zombie().clone();

        if lich_zombie.to_be_zombified.contains(&dead_player) {
            dead_player.set_role(game, RoleState::Zombie(Zombie::default()));
            lich_zombie.to_be_zombified.remove(&dead_player);
            game.set_lich_zombie(lich_zombie);
        }
    }
    pub fn on_game_ending(game: &mut Game){
        let mut lich_zombie: LichZombie = game.lich_zombie().clone();

        for zombie in lich_zombie.to_be_zombified.clone(){
            if zombie.role(game) == Role::Zombie {continue;}
            zombie.set_role(game, RoleState::Zombie(Zombie::default()));
            lich_zombie.to_be_zombified.remove(&zombie);
        }
        game.set_lich_zombie(lich_zombie);
    }
}

impl Game{
    pub fn lich_zombie<'a>(&'a self)->&'a LichZombie{
        &self.lich_zombie
    }
    pub fn set_lich_zombie(&mut self, lich_zombie: LichZombie){
        self.lich_zombie = lich_zombie;
    }
}