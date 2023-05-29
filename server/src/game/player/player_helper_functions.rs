use crate::game::{chat::{ChatMessage, night_message::NightInformation}, Game, grave::GraveKiller, role::RoleData};

use super::{Player, PlayerReference};



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
    pub fn set_role(&self, game: &mut Game, new_role_data: RoleData){

        self.set_role_data(game, new_role_data);
        self.role(game).on_role_creation(game, *self);
        let chat_message = ChatMessage::RoleAssignment { role: self.role(game)};
        self.add_chat_message(game, chat_message);
    }
    pub fn increase_defense_to(&self, game: &mut Game, defense: u8){
        if self.night_defense(game) < defense {
            self.set_night_defense(game, defense);
        }
    }
    
}



