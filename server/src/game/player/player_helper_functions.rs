use crate::game::{chat::{ChatMessage, night_message::NightInformation}, Game, grave::GraveKiller, role::RoleData};

use super::{Player, PlayerReference};



impl Player{
    ///returns true if they were roleblocked by you
    pub fn roleblock(&mut self)->bool{
        if self.role().roleblockable() {
            self.set_night_roleblocked(true);
            self.push_night_messages(
                ChatMessage::NightInformation { night_information: NightInformation::RoleBlocked { immune: false }}
            );
            return true;
        }else{
            self.push_night_messages(
                ChatMessage::NightInformation { night_information: NightInformation::RoleBlocked { immune: true }}
            );
            return false;
        }
    }
    ///returns true if attack overpowered defense.
    pub fn try_night_kill(game: &mut Game, player_ref: PlayerReference, grave_killer: GraveKiller, attack: u8)->bool{
        let player = player_ref.deref_mut(game);

        player.set_night_attacked(true);

        if *player.night_defense() >= attack {
            player.push_night_messages(
                ChatMessage::NightInformation { night_information: NightInformation::YouSurvivedAttack }
            );
            return false;
        }
        
        //die
        player.push_night_messages(ChatMessage::NightInformation { night_information: NightInformation::YouDied });
        player.set_night_died(true);
        Player::set_alive(game, player_ref, false);
        player_ref.deref_mut(game).push_night_grave_killers(grave_killer);

        true
    }
    /// swap this persons role, sending them the role chat message, and associated changes
    pub fn set_role(game: &mut Game, player_ref: PlayerReference, new_role_data: RoleData){

        player_ref.deref_mut(game).set_role_data(new_role_data);
        player_ref.deref(game).role().on_role_creation(game,player_ref);
        let chat_message = ChatMessage::RoleAssignment { role: player_ref.deref(game).role()};
        player_ref.deref_mut(game).add_chat_message(chat_message);
    }
    pub fn increase_defense_to(&mut self, defense: u8){
        if self.night_variables.defense < defense{
            self.night_variables.defense = defense;
        }
    }
    
}



