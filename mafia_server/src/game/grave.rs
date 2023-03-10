use serde::{Serialize, Deserialize};

use super::Game;
use super::player::PlayerIndex;
use super::role::Role;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grave {
    player: PlayerIndex,

    role: GraveRole,
    death_cause: GraveDeathCause,
    will: String,

    died_phase: GravePhase,
    day_number: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GraveRole {
    Cleaned,
    Stoned,
    Role(Role),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GraveDeathCause {
    Lynching,
    Killers{killers: Vec<GraveKiller>}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GraveKiller {
    Mafia,
    Role(Role)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GravePhase {
    Day, Night
}

impl Grave{
    pub fn from_player_night(game: &mut Game, player_index: PlayerIndex)->Grave{

        let player = game.get_unchecked_mut_player(player_index);

        Grave { 
            player: player_index, 
            role: player.night_variables.grave_role.clone(),
            death_cause: GraveDeathCause::Killers {killers: player.night_variables.grave_killers.clone()}, 
            will: player.night_variables.grave_will.clone(),
            died_phase: GravePhase::Night, 
            day_number: game.phase_machine.day_number
        }
    }
    pub fn from_player_lynch(game: &mut Game, player_index: PlayerIndex)->Grave{

        let player = game.get_unchecked_mut_player(player_index);

        Grave { 
            player: player_index, 
            role: GraveRole::Role(player.get_role()), 
            death_cause: GraveDeathCause::Lynching, 
            will: player.will.clone(), 
            died_phase: GravePhase::Day, 
            day_number: game.phase_machine.day_number
        }
    }
}