use serde::Serialize;

use crate::{game::{chat::ChatMessageVariant, player::PlayerReference, role::{armorsmith::Armorsmith, Role}, tag::Tag, Game}, vec_set::VecSet};

use super::{confused::Confused, detained::Detained, love_linked::LoveLinked};
#[derive(Debug, Clone, Default)]
pub struct PathologistInfoDump;


impl PathologistInfoDump {
    /// Here rather than in Pathologist because it must run before any other step in the player death invocation
    /// because it needs to gather information as if the player was still alive.
    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference) {
        let status_message = Self::new_status_effects_message(game, dead_player, true);
        let convert_message = ChatMessageVariant::PlayerConvertHistory {
            player: dead_player.index(),
            history: game.convert_history(dead_player),
        };
        let role_will_message = ChatMessageVariant::PlayerRoleAndAlibi { 
            player: dead_player, 
            role: dead_player.role(game), 
            will: dead_player.will(game).to_owned(),
        };

        let pathologists = PlayerReference::all_players(game).filter(|player| 
            player.role(game) == Role::Pathologist
        ).collect::<Box<[PlayerReference]>>();

        if game.current_phase().is_night() {
            for pathologist in pathologists {
                pathologist.push_night_message(game, status_message.clone());
                pathologist.push_night_message(game, convert_message.clone());
                pathologist.push_night_message(game, role_will_message.clone());
            }
        } else {
            for pathologist in pathologists {
                pathologist.add_private_chat_message(game, status_message.clone());
                pathologist.add_private_chat_message(game, convert_message.clone());
                pathologist.add_private_chat_message(game, role_will_message.clone());
            }
        }
    }
    /// The reason why the specify target option exists, but is always true is for future proofing, 
    /// In case a different role is given the ability to learn this info, but it's based on who they visit.
    pub fn new_status_effects_message(game: &Game, target: PlayerReference, specify_target: bool) -> ChatMessageVariant{
        if game.current_phase().is_night() {
            ChatMessageVariant::PlayerStatusEffects {
                player: if specify_target {Some(target.index())} else {None},
                tags: target.tags_on_player(game).into_iter().flat_map(|t|t.1).collect::<VecSet<Tag>>().into_iter().collect(),
                love_links: LoveLinked::get_links(game, target).iter().map(|p|p.index()).collect(),
                innocent_aura: target.has_innocent_aura(game),
                sus_aura: target.has_suspicious_aura(game),
                armor: Armorsmith::player_has_armor(game, &target),
                confused: Confused::is_confused(game, target),
                silenced: target.night_silenced(game),
                night_status: Some(NightStatusEffects {
                    night_defense: target.night_defense(game) as u8,
                    roleblocked: target.night_roleblocked(game),
                    wardblocked: target.night_wardblocked(game),
                    possessed: target.night_possessed(game),
                    transported: target.night_transported(game),
                    detained: Detained::is_detained(game, target),
                }),
            }
        } else {
            ChatMessageVariant::PlayerStatusEffects {
                player: if specify_target {Some(target.index())} else {None},
                tags: target.tags_on_player(game).into_iter().flat_map(|t|t.1).collect(),
                love_links: LoveLinked::get_links(game, target).iter().map(|p|p.index()).collect(),
                innocent_aura: target.has_innocent_aura(game),
                sus_aura: target.has_suspicious_aura(game),
                armor: Armorsmith::player_has_armor(game, &target),
                confused: Confused::is_confused(game, target),
                silenced: target.night_silenced(game),
                night_status: None,
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct NightStatusEffects{
    night_defense: u8,
    roleblocked: bool,
    wardblocked: bool,
    possessed: bool,
    transported: bool,
    detained: bool,
}