use crate::game::{chat::{ChatMessage, night_message::NightInformation, ChatGroup}, Game, grave::GraveKiller, role::{RoleState, Priority}, visit::Visit};

use super::PlayerReference;



impl PlayerReference{
    ///returns true if they were roleblocked by you
    pub fn roleblock(&self, game: &mut Game)->bool{
        if self.role(game).roleblockable() {
            self.set_night_roleblocked(game, true);
            self.set_night_visits(game, vec![]);
            self.push_night_messages(game,
                NightInformation::RoleBlocked { immune: false }
            );
            true
        } else {
            self.push_night_messages(game,
                NightInformation::RoleBlocked { immune: true }
            );
            false
        }
    }
    ///returns true if attack overpowered defense.
    pub fn try_night_kill(&self, game: &mut Game, grave_killer: GraveKiller, attack: u8)->bool{
        self.set_night_attacked(game, true);

        if self.night_defense(game) >= attack {
            self.push_night_messages(game,
                NightInformation::YouSurvivedAttack
            );
            return false;
        }
        
        //die
        self.push_night_messages(game, NightInformation::YouDied);

        if !self.alive(game){
            return true;
        }
        self.set_night_died(game, true);
        self.set_alive(game, false);
        self.push_night_grave_killers(game, grave_killer);

        true
    }
    /// swap this persons role, sending them the role chat message, and associated changes
    pub fn set_role(&self, game: &mut Game, new_role_data: RoleState){

        self.set_role_state(game, new_role_data);
        self.on_role_creation(game);
        let chat_message = ChatMessage::RoleAssignment { role: self.role(game)};
        self.add_chat_message(game, chat_message);
    }
    pub fn increase_defense_to(&self, game: &mut Game, defense: u8){
        if self.night_defense(game) < defense {
            self.set_night_defense(game, defense);
        }
    }
    pub fn can_night_target(&self, game: &Game, target_ref: PlayerReference) -> bool {
        self.role_state(game).clone().can_night_target(game, *self, target_ref)
    }
    pub fn can_day_target(&self, game: &Game, target_ref: PlayerReference) -> bool {
        self.role_state(game).clone().can_day_target(game, *self, target_ref)
    }
    pub fn do_night_action(&self, game: &mut Game, priority: Priority) {
        self.role_state(game).clone().do_night_action(game, *self, priority)
    }
    pub fn do_day_action(&self, game: &mut Game, target_ref: PlayerReference) {
        self.role_state(game).clone().do_day_action(game, *self, target_ref)
    }
    pub fn on_role_creation(&self, game: &mut Game) {
        self.role_state(game).clone().on_role_creation(game, *self)
    }
    pub fn get_current_send_chat_groups(&self, game: &Game) -> Vec<ChatGroup> {
        self.role_state(game).clone().get_current_send_chat_groups(game, *self)
    }
    pub fn get_current_recieve_chat_groups(&self, game: &Game) -> Vec<ChatGroup> {
        self.role_state(game).clone().get_current_recieve_chat_groups(game, *self)
    }
    pub fn convert_targets_to_visits(&self, game: &Game, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        self.role_state(game).clone().convert_targets_to_visits(game, *self, target_refs)
    }
}



