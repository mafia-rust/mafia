use crate::{game::{grave::Grave, player::PlayerReference, role::Role, Game}, vec_map::VecMap};

/*
 * Not a role state based win condition because the player has to 
 */
#[derive(Default, Clone)]
pub struct MimicWinCon{
    players_have_won: VecMap<PlayerReference, bool>
}

impl Game {
    pub fn mimic_win_con(&self)->&MimicWinCon{
        &self.mimic_win_con
    }
    pub fn mimic_win_con_mut(&mut self)->&mut MimicWinCon{
        &mut self.mimic_win_con
    }

}


impl MimicWinCon {
    pub fn has_won(game: &Game, player: PlayerReference) -> bool{
        return game.mimic_win_con().players_have_won.contains_key(&player);
    }
    pub fn new_mimic(game: &mut Game, player: PlayerReference) {
        game.mimic_win_con_mut().players_have_won.insert(player, false);
    }

    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference) {
        if dead_player.role(game) != Role::Mimic {return};
        let mimic_win_con_clone = game.mimic_win_con().clone();
        let mut players_have_won_clone = mimic_win_con_clone.players_have_won.clone();

        for players_have_won in mimic_win_con_clone.players_have_won {
            players_have_won_clone.insert(players_have_won.0, players_have_won.0.alive(game) || players_have_won.1);
            players_have_won.0.die(game, Grave::from_player_leave_town(game, players_have_won.0));
        }

        game.mimic_win_con.players_have_won = players_have_won_clone;
    }
}