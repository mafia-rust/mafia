use crate::game::{
    components::{
        detained::Detained,
        mafia_recruits::MafiaRecruits,
        pitchfork::Pitchfork, poison::Poison,
        puppeteer_marionette::PuppeteerMarionette
    }, modifiers::Modifiers, role::Priority, Game
};

///runs before all players' night actions
#[must_use = "Event must be invoked"]
pub struct OnNightPriority{
    priority: Priority
}
impl OnNightPriority{
    pub fn new(priority: Priority) -> Self{
        Self{priority}
    }
    pub fn invoke(self, game: &mut Game){
        Detained::on_night_priority(game, self.priority);
        Poison::on_night_priority(game, self.priority);
        PuppeteerMarionette::on_night_priority(game, self.priority);
        MafiaRecruits::on_night_priority(game, self.priority);
        Pitchfork::on_night_priority(game, self.priority);
        Modifiers::on_night_priority(game, self.priority);
    }
}