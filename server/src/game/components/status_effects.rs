use serde::Serialize;

use crate::game::{attack_power::DefensePower, player::{PlayerIndex, PlayerReference}, role::armorsmith::Armorsmith, tag::Tag, Game};

use super::{detained::Detained, love_linked::LoveLinked, poison::Poison};



/// For message purposes only. The data stored by this 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusEffects {
    pub tags: Vec<Tag>,
    pub love_links: Vec<PlayerIndex>,
    pub innocent_aura: bool,
    pub sus_aura: bool,
    pub armor: bool,
    pub spiraling: bool,
    /* night only */
    pub night_defense: DefensePower,
    pub silenced: bool,
    pub roleblocked: bool,
    pub wardblocked: bool,
    pub possessed: bool,
    pub transported: bool,
    pub detained: bool,
}

impl StatusEffects {
    pub fn new(game: &Game, player: PlayerReference) -> StatusEffects {
        if game.current_phase().is_night() {
            StatusEffects {
                tags: player.tags_on_player(game).into_iter().flat_map(|t|t.1).collect(),
                love_links: LoveLinked::get_links(game, player).iter().map(|p|p.index()).collect(),
                innocent_aura: player.has_innocent_aura(game),
                sus_aura: player.has_suspicious_aura(game),
                armor: Armorsmith::player_has_armor(game, &player),
                spiraling: Poison::is_spiraling(game, player),
                /* night only */
                night_defense: player.night_defense(game),
                silenced: player.night_silenced(game),
                roleblocked: player.night_roleblocked(game),
                wardblocked: player.night_wardblocked(game),
                possessed: player.night_possessed(game),
                transported: player.night_transported(game),
                detained: Detained::is_detained(game, player),
            }
        } else {
            StatusEffects {
                tags: player.tags_on_player(game).into_iter().flat_map(|t|t.1).collect(),
                love_links: LoveLinked::get_links(game, player).iter().map(|p|p.index()).collect(),
                innocent_aura: player.has_innocent_aura(game),
                sus_aura: player.has_suspicious_aura(game),
                armor: Armorsmith::player_has_armor(game, &player),
                spiraling: Poison::is_spiraling(game, player),
                /* night only */
                night_defense: DefensePower::None,
                silenced: false,
                roleblocked: false,
                wardblocked: false,
                possessed: false,
                transported: false,
                detained: false,
            }
        }
    }
}