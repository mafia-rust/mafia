use crate::game::{
    grave::Grave,
    player::PlayerReference,
    Game
};

#[derive(Default, Clone)]
pub struct Ascension;
impl Ascension{
    pub fn on_role_switch(game: &mut Game){
        Ascension::ascend_players(game);
    }
    pub fn on_any_death(game: &mut Game){
        Ascension::ascend_players(game);
    }
    pub fn on_game_start(game: &mut Game){
        Ascension::ascend_players(game);
    }

    fn ascend_players(game: &mut Game){
        for player in PlayerReference::all_players(game) {
            if Ascension::should_ascend_player(game, player){
                Ascension::ascend_player(game, player);
            }
        }
    }

    fn ascend_player(game: &mut Game, player: PlayerReference){
        let new_grave = Grave::from_player_ascension(game, player);
        player.die(game, new_grave);
    }

    //Example use is minion/scarecrow. This can detect if they won early.
    fn should_ascend_player(game: &Game, player: PlayerReference) -> bool {

        let Some(set) = player.required_resolution_states_for_win(game) else {return false};

        for player in PlayerReference::all_players(game){
            if !player.keeps_game_running(game) {continue};

            if let Some(player_set) = player.required_resolution_states_for_win(game){
                if !player_set.is_superset(&set){
                    return false;
                }
            }
        }
        if !player.alive(game) {return false;}
        true
    }
    
}