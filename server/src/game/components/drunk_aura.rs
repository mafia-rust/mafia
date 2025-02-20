use crate::game::{phase::PhaseType, player::PlayerReference, role::Role, Game};

use super::{duration::Duration, confused::Confused};

#[derive(Default, Clone)]
pub struct DrunkAura {
    pub players: Vec<PlayerReference>,
    pub durations: Vec<Duration>,
}

impl DrunkAura {
    fn drunk_aura(game: &Game) -> &Self {
        &game.drunk_aura
    }
    fn drunk_aura_mut(game: &mut Game) -> &mut Self {
        &mut game.drunk_aura
    }

    pub fn add_player_permanent(game: &mut Game, player: PlayerReference) {
        let drunk_aura = Self::drunk_aura_mut(game);
        drunk_aura.players.push(player);
        drunk_aura.durations.push(Duration::Permanent)
    }
    
    pub fn add_player(game: &mut Game, player: PlayerReference, duration: u8){
        let drunk_aura = Self::drunk_aura_mut(game);
        drunk_aura.players.push(player);
        drunk_aura.durations.push(Duration::Temporary { duration })
    }

    pub fn remove_player(game: &mut Game, player: PlayerReference) {
        let drunk_aura = Self::drunk_aura_mut(game);
        let index = drunk_aura.players.iter().position(|x| *x == player);
        match index {
            Some(i) => {
                drunk_aura.durations.swap_remove(i); 
                drunk_aura.players.swap_remove(i);
            },
            None => (),
        }
    }
    
    pub fn has_drunk_aura(game: &Game, player: PlayerReference) -> bool {
        Self::drunk_aura(game).players.contains(&player)
    }

    pub fn on_role_switch(game: &mut Game, player: PlayerReference, old_role: Role) {
        if Self::has_drunk_aura(game, player) {
            match old_role {
                Role::Wildcard | Role::MafiaKillingWildcard | Role::MafiaSupportWildcard | Role::TrueWildcard | Role::FiendsWildcard
                => (),
                _ =>  {
                    Self::remove_player(game, player);
                    Confused::remove_player(game, player);
                }
            }
            
        }
    }

    ///Decrements drunk aura durations and removes players whose durations are up
    pub fn before_phase_end(game: &mut Game, phase: PhaseType){
        match phase {
            PhaseType::Dusk=>{
                let duration_max_index = game.drunk_aura.durations.len()-1;
                for i in 0..duration_max_index {
                    let index = duration_max_index - i;
                    let player: PlayerReference = game.drunk_aura.players[i];
                    if player.role(game).any_wildcard() {
                        if !game.drunk_aura.durations[index].is_over() {
                            game.drunk_aura.players.swap_remove(index);
                            game.drunk_aura.durations.swap_remove(index);
                        }
                    } else {
                        if !game.drunk_aura.durations[index].decrement() {
                            game.drunk_aura.players.swap_remove(index);
                            game.drunk_aura.durations.swap_remove(index);
                        }
                    }
                }
            },
            _=>{}
        }
    }
}
