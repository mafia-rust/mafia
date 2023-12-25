use std::vec;

use serde::{Serialize, Deserialize};

use super::Game;
use super::player::{PlayerIndex, PlayerReference};
use super::role::Role;
use super::role_list::Faction;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Grave {
    #[serde(rename = "playerIndex")]
    pub player: PlayerIndex,

    pub role: GraveRole,
    pub death_cause: GraveDeathCause,
    pub will: String,
    pub death_notes: Vec<String>,

    pub died_phase: GravePhase,
    pub day_number: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "role")]
pub enum GraveRole {
    Cleaned,
    Petrified,
    Role(Role),
}
impl GraveRole{
    pub fn get_role(&self)->Option<Role>{
        match self {
            GraveRole::Role(role) => Some(*role),
            _ => None
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "killers")]
pub enum GraveDeathCause {
    Lynching,
    DisconnectedFromLife,
    Killers(Vec<GraveKiller>)
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "value")]
pub enum GraveKiller {
    Faction(Faction),
    Role(Role),
    Suicide
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GravePhase {
    Day,
    Night
}

impl Grave{
    pub fn from_player_night(game: &Game, player_ref: PlayerReference) -> Grave {
        let day_number = game.phase_machine.day_number;

        Grave { 
            player: player_ref.index(), 
            role: player_ref.night_grave_role(game).clone(),
            death_cause: GraveDeathCause::Killers(player_ref.night_grave_killers(game).clone()),
            will: player_ref.night_grave_will(game).clone(),
            died_phase: GravePhase::Night, 
            day_number,
            death_notes: player_ref.night_grave_death_notes(game).clone()
        }
    }
    pub fn from_player_lynch(game: &Game, player_ref: PlayerReference) -> Grave {

        Grave { 
            player: player_ref.index(), 
            role: GraveRole::Role(player_ref.role(game)), 
            death_cause: GraveDeathCause::Lynching, 
            will: player_ref.will(game).clone(), 
            died_phase: GravePhase::Day, 
            day_number: game.phase_machine.day_number,
            death_notes: vec![]
        }
    }

    pub fn from_player_suicide(game: &Game, player_ref: PlayerReference) -> Grave {
        Grave {
            player: player_ref.index(), 
            role: GraveRole::Role(player_ref.role(game)), 
            death_cause: GraveDeathCause::Killers(vec![GraveKiller::Suicide]), 
            will: player_ref.will(game).clone(), 
            died_phase: GravePhase::Day, 
            day_number: game.phase_machine.day_number,
            death_notes: vec![]
        }
    }
}