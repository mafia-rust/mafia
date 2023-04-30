use std::default;

use crate::game::{
    visit::Visit, verdict::Verdict, Game, role::Role, 
    chat::{night_message::NightInformation, ChatMessage}, 
    grave::{GraveRole, GraveDeathCause, GraveKiller}
};

use super::{PlayerIndex, Player, player};
pub struct PlayerNightVariables{
    pub alive_tonight:  bool,
    pub died:           bool,
    pub attacked:       bool,
    pub roleblocked:    bool,
    pub defense:        u8,    
    pub suspicious:     bool,

    pub disguised_as:   PlayerIndex,

    pub chosen_targets: Vec<PlayerIndex>,
    pub visits:         Vec<Visit>,

    pub night_messages: Vec<ChatMessage>,

    pub grave_role: GraveRole,
    pub grave_killers: Vec<GraveKiller>,
    pub grave_will: String,
    pub grave_death_notes: Vec<String>

}
impl Default for PlayerNightVariables{
    fn default() -> Self {
        Self{
            alive_tonight:  true,
            died:           false,
            attacked:       false,
            roleblocked:    false,
            defense:        0,
            suspicious:     false,

            disguised_as:   0,

            chosen_targets: vec![],
            visits:         vec![],

            night_messages: vec![],

            grave_role: GraveRole::Role(Role::Sheriff), //This should not be a problem because we reset immedietly on creation
            grave_killers: vec![],
            grave_will: "".to_string(),
            grave_death_notes: vec![],
        }
    }
}
impl PlayerNightVariables{
    pub fn new()->Self{
        Self{
            alive_tonight:  true,
            died:           false,
            attacked:       false,
            roleblocked:    false,
            defense:        0,
            suspicious:     false,

            disguised_as:   0,

            chosen_targets: vec![],
            visits:         vec![],

            night_messages: vec![],

            grave_role: GraveRole::Role(Role::Sheriff), //This should not be a problem because we reset immedietly on creation
            grave_killers: vec![],
            grave_will: "".to_string(),
            grave_death_notes: vec![],
        }
    }
    pub fn reset(game: &Game, player_index: PlayerIndex)->Self{
        let player = game.get_unchecked_player(player_index);
        return Self{
            alive_tonight:  player.alive,
            died:           false,
            attacked:       false,
            roleblocked:    false,
            defense:        player.role().get_defense(),
            suspicious:     player.role().is_suspicious(),

            disguised_as:   player_index,

            chosen_targets: vec![],
            visits:         vec![],

            night_messages: vec![],

            grave_role: GraveRole::Role(player.role()),
            grave_killers: vec![],
            grave_will: player.will().clone(),
            grave_death_notes: vec![],
        };
    }
    pub fn increase_defense_to(&mut self, defense: u8){
        if self.defense < defense{
            self.defense = defense;
        }
    }
}