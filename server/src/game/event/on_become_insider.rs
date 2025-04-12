use crate::game::{
    components::{
        insider_group::InsiderGroupID, mafia::Mafia,
        mafia_recruits::MafiaRecruits, puppeteer_marionette::PuppeteerMarionette,
        syndicate_gun_item::SyndicateGunItem
    },
    player::PlayerReference
};
use super::Event;

#[derive(Clone)]
pub struct OnBecomeInsider {
    pub player: PlayerReference,
    pub group: InsiderGroupID,
    pub on_init: bool
}

impl OnBecomeInsider {
    pub fn new(player: PlayerReference, group: InsiderGroupID, on_init: bool) -> Self {
        Self {
            player,
            group,
            on_init
        }
    }
}

impl Event for OnBecomeInsider {
    type FoldValue = ();
    type Priority = ();

    fn listeners() -> Vec<super::EventListenerFunction<Self>> {
        vec![
            SyndicateGunItem::on_become_insider,
            PuppeteerMarionette::on_become_insider,
            Mafia::on_become_insider,
            MafiaRecruits::on_become_insider,
        ]
    }
    
    fn initial_fold_value(&self) -> Self::FoldValue {}
}