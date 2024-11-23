use rand::seq::SliceRandom;

use crate::game::{components::{insider_group::InsiderGroupID, syndicate_gun_item}, player::PlayerReference};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct SyndicateGunItem;

/*
    There is modifier specific code in the common_role::get_send_chat_groups() function
*/
impl From<&SyndicateGunItem> for ModifierType{
    fn from(_: &SyndicateGunItem) -> Self {
        ModifierType::SyndicateGunItem
    }
}

impl ModifierTrait for SyndicateGunItem{

    // this should also happen whenever someone becomes an insider, but that cant happen yet, cant go from 0 insiders to > 0 insiders
    fn on_game_start(self, game: &mut crate::game::Game) {
        //give random syndicate insider the gun
        let insiders = PlayerReference::all_players(game)
            .filter(|p| InsiderGroupID::Mafia.is_player_in_revealed_group(game, *p))
            .collect::<Vec<_>>();

        let Some(insider) = insiders.choose(&mut rand::thread_rng()) else {return};

        syndicate_gun_item::SyndicateGunItem::give_gun(game, *insider);
    }
}