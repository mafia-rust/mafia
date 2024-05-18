use crate::game::{components::puppeteer_marionette::PuppeteerMarionette, role::Priority, Game};

///runs before all players' night actions
pub struct OnNightPriority{
    priority: Priority
}
impl OnNightPriority{
    pub fn new(priority: Priority) -> Self{
        Self{priority}
    }
    pub fn invoke(self, game: &mut Game){
        PuppeteerMarionette::on_night_priority(game, self.priority);
    }
}