use crate::game::{player::PlayerReference, role::{zombie::Zombie, Role, RoleState}, Game};

#[derive(Default, Clone)]
pub struct LichZombie{
    to_be_zombified: Vec<PlayerReference>
}
impl LichZombie{
    pub fn zombify(game: &mut Game, player: PlayerReference){
        let mut lich_zombie = game.lich_zombie().clone();
        lich_zombie.to_be_zombified.push(player);
        game.set_lich_zombie(lich_zombie)
    }
    pub fn kill_zombies(game: &mut Game){
        for zombie in game.lich_zombie().to_be_zombified.clone() {
            for lich in PlayerReference::all_players(game){
                zombie.try_night_kill(lich, game, crate::game::grave::GraveKiller::Role(Role::Lich), 2, true);
            }
        }
    }
    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference){
        let mut lich_zombie: LichZombie = game.lich_zombie().clone();

        if lich_zombie.to_be_zombified.contains(&dead_player) {
            dead_player.set_role(game, RoleState::Zombie(Zombie::default()));
            lich_zombie.to_be_zombified.retain(|f|*f!=dead_player);
            game.set_lich_zombie(lich_zombie);
        }
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