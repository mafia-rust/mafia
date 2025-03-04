use crate::game::{chat::ChatMessageVariant, player::PlayerReference, role::Role, Game};

use super::status_effects::StatusEffects;

#[derive(Debug, Clone, Default)]
pub struct PathologistInfoDump;


impl PathologistInfoDump {
    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference) {
        let convert_message = ChatMessageVariant::PlayerConvertHistory {
            player: dead_player.index(),
            history: game.convert_history(dead_player),
        };
        let status_message = ChatMessageVariant::PlayerStatus { 
            player: dead_player.index(),
            status: StatusEffects::new(game, dead_player),
        };

        let pathologists = PlayerReference::all_players(game).filter(|player| 
            player.role(game) == Role::Pathologist
        ).collect::<Box<[PlayerReference]>>();

        if game.current_phase().is_night() {
            for pathologist in pathologists {
                pathologist.push_night_message(game, convert_message.clone());
                pathologist.push_night_message(game, status_message.clone());
            }
        } else {
            for pathologist in pathologists {
                pathologist.add_private_chat_message(game, convert_message.clone());
                pathologist.add_private_chat_message(game, status_message.clone());
            }
        }
    }
}