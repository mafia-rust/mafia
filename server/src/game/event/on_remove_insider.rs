use crate::game::{
    components::{
        insider_group::InsiderGroupID, mafia::Mafia,
        mafia_recruits::MafiaRecruits, puppeteer_marionette::PuppeteerMarionette,
        syndicate_gun_item::SyndicateGunItem
    },
    player::PlayerReference, Game
};
use super::Event;

#[derive(Clone)]
pub struct OnRemoveInsider {
    pub player: PlayerReference,
    pub group: InsiderGroupID,
}

impl OnRemoveInsider {
    pub fn new(player: PlayerReference, group: InsiderGroupID) -> Self {
        Self {
            player,
            group,
        }
    }
}

impl Event for OnRemoveInsider {
    type FoldValue = ();
    type Priority = ();

    fn listeners() -> Vec<super::EventListenerFunction<Self>> {
        vec![
            SyndicateGunItem::on_remove_insider,
            PuppeteerMarionette::on_remove_insider,
            Mafia::on_remove_insider,
            MafiaRecruits::on_remove_insider,
        ]
    }
    
    fn initial_fold_value(&self, _game: &Game) -> Self::FoldValue {}
}