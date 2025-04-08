use crate::{event_priority, game::{
    components::{
        detained::Detained, mafia::Mafia, mafia_recruits::MafiaRecruits,
        pitchfork::Pitchfork, poison::Poison, puppeteer_marionette::PuppeteerMarionette,
        syndicate_gun_item::SyndicateGunItem
    }, modifiers::Modifiers, player::PlayerReference
}};
use super::Event;

///runs before all players' night actions
#[must_use = "Event must be invoked"]
pub struct OnMidnight;
event_priority!(OnMidnightPriority{
    TopPriority,
    Ward,

    Transporter,
    Warper,

    Possess,
    Roleblock,

    Deception,

    Bodyguard,

    Heal,
    Kill,
    Convert,    //role swap & win condition change
    Poison,
    Investigative,

    Cupid,
    SpyBug,

    StealMessages
});

impl OnMidnight{
    pub fn new() -> Self{Self{}}
}
impl Event for OnMidnight {
    type FoldValue = ();
    type Priority = OnMidnightPriority;

    fn listeners() -> Vec<super::EventListenerFunction<Self>> {
        vec![
            Detained::on_midnight,
            Poison::on_midnight,
            PuppeteerMarionette::on_midnight,
            MafiaRecruits::on_midnight,
            Pitchfork::on_midnight,
            Modifiers::on_midnight,
            SyndicateGunItem::on_midnight,
            Mafia::on_midnight,
            
            PlayerReference::on_midnight,
        ]
    }

    fn initial_fold_value(&self) -> Self::FoldValue {}
}
/*
    pub fn on_midnight(game: &mut Game, _event: OnMidnight, _fold: (), priority: OnMidnightPriority){
 */