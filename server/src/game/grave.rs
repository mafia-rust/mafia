use serde::{Serialize, Deserialize};

use super::Game;
use super::player::PlayerIndex;
use super::role::Role;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grave {
    #[serde(rename = "playerIndex")]
    player: PlayerIndex,

    role: GraveRole,
    death_cause: GraveDeathCause,
    will: String,

    died_phase: GravePhase,
    day_number: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "role")]
pub enum GraveRole {
    Cleaned,
    Stoned,
    Role(Role),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "killers")]
pub enum GraveDeathCause {
    Lynching,
    Killers(Vec<GraveKiller>)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "role")]
pub enum GraveKiller {
    Mafia,
    Role(Role)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GravePhase {
    Day, 
    Night
}

impl Grave{
    pub fn from_player_night(game: &mut Game, player_index: PlayerIndex)->Grave{

        let player = game.get_unchecked_mut_player(player_index);

        Grave { 
            player: player_index, 
            role: player.night_variables.grave_role.clone(),
            death_cause: GraveDeathCause::Killers(player.night_variables.grave_killers.clone()), 
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