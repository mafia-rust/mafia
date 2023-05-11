use std::vec;

use serde::{Serialize, Deserialize};

use super::Game;
use super::player::{PlayerIndex, PlayerReference};
use super::role::Role;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grave {
    #[serde(rename = "playerIndex")]
    player: PlayerIndex,

    role: GraveRole,
    death_cause: GraveDeathCause,
    will: String,
    death_notes: Vec<String>,

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
    pub fn from_player_night(game: &mut Game, player_ref: PlayerReference)->Grave{

        let day_number = game.phase_machine.day_number.clone();

        Grave { 
            player: *player_ref.index(), 
            role: player_ref.night_grave_role(game).clone(),
            death_cause: GraveDeathCause::Killers(player_ref.night_grave_killers(game).clone()),
            will: player_ref.night_grave_will(game).clone(),
            died_phase: GravePhase::Night, 
            day_number,
            death_notes: player_ref.night_grave_death_notes(game).clone()
        }
    }
    pub fn from_player_lynch(game: &mut Game, player_ref: PlayerReference)->Grave{

        Grave { 
            player: *player_ref.index(), 
            role: GraveRole::Role(player_ref.role(game)), 
            death_cause: GraveDeathCause::Lynching, 
            will: player_ref.will(game).clone(), 
            died_phase: GravePhase::Day, 
            day_number: game.phase_machine.day_number,
            death_notes: vec![]
        }
    }
}