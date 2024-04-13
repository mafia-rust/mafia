use crate::game::
{
    chat::{ChatGroup, ChatMessageVariant}, end_game_condition::EndGameCondition, event::{on_any_death::OnAnyDeath, on_role_switch::OnRoleSwitch}, grave::{Grave, GraveKiller}, role::{same_evil_team, Priority, Role, RoleState}, visit::Visit, Game
};

use super::PlayerReference;

impl PlayerReference{
    pub fn roleblock(&self, game: &mut Game, send_messages: bool) {
        if !self.role(game).roleblock_immune() {
            self.set_night_roleblocked(game, true);
            self.set_night_visits(game, vec![]);
            
            if send_messages {
                self.push_night_message(game,
                    ChatMessageVariant::RoleBlocked { immune: false }
                );
            }
        } else if send_messages {
            self.push_night_message(game,
                ChatMessageVariant::RoleBlocked { immune: true }
            );
        }
    }

    /// Returns true if attack overpowered defense
    pub fn try_night_kill(&self, attacker_ref: PlayerReference, game: &mut Game, grave_killer: GraveKiller, attack: u8, should_leave_death_note: bool) -> bool {
        self.set_night_attacked(game, true);

        if self.night_defense(game) >= attack {
            self.push_night_message(game,
                ChatMessageVariant::YouSurvivedAttack
            );
            attacker_ref.push_night_message(game,ChatMessageVariant::SomeoneSurvivedYourAttack);
            return false;
        }

        self.push_night_grave_killers(game, grave_killer);
        if should_leave_death_note {
            if let Some(note) = attacker_ref.death_note(game) {
                self.push_night_grave_death_notes(game, note.clone());
            }
        }
        

        if !self.alive(game) { return true }

        self.set_night_died(game, true);

        true
    }

    /// ### Pre condition:
    /// self.alive(game) == false
    pub fn die(&self, game: &mut Game, grave: Grave){
        self.die_return_event(game, grave).invoke(game);
    }
    pub fn die_return_event(&self, game: &mut Game, grave: Grave)->OnAnyDeath{
        self.set_alive(game, false);
        self.add_private_chat_message(game, ChatMessageVariant::YouDied);
        game.add_grave(grave.clone());

        return OnAnyDeath::new(*self);
    }
    /// Swaps this persons role, sends them the role chat message, and makes associated changes
    pub fn set_role(&self, game: &mut Game, new_role_data: RoleState){

        self.set_role_state(game, new_role_data.clone());
        self.on_role_creation(game);
        if new_role_data.role() == self.role(game) {
            self.add_private_chat_message(game, ChatMessageVariant::RoleAssignment{role: self.role(game)});
        }

        self.insert_role_label(game, *self, self.role(game));
        OnRoleSwitch::new(*self).invoke(game);
    }
    pub fn increase_defense_to(&self, game: &mut Game, defense: u8){
        if self.night_defense(game) < defense {
            self.set_night_defense(game, defense);
        }
    }
    
    
    pub fn tracker_seen_visits(self, game: &Game) -> Vec<Visit> {
        if let Some(v) = self.night_appeared_visits(game) {
            v.clone()
        } else {
            self.night_visits(game).clone()
        }
    }
    pub fn appeared_visitors(self, game: &Game) -> Vec<PlayerReference> {
        PlayerReference::all_players(game).filter(|player_ref|{
            player_ref.tracker_seen_visits(game).iter().any(|other_visit| 
                other_visit.target == self
            )
        }).collect()
    }
    pub fn all_visitors(self, game: &Game) -> Vec<PlayerReference> {
        PlayerReference::all_players(game).filter(|player_ref|{
            player_ref.night_visits(game).iter().any(|other_visit| 
                other_visit.target == self
            )
        }).collect()
    }



    pub fn insert_role_label_for_teammates(&self, game: &mut Game){
        let actor_role = self.role(game);
    
        for other in PlayerReference::all_players(game){
            if *self == other { continue }
            
            if same_evil_team(game, *self, other) {
                let other_role = other.role(game);
                other.insert_role_label(game, *self, actor_role);
                self.insert_role_label(game, other, other_role);
            }
        }
    }


    pub fn defense(&self, game: &Game) -> u8 {
        if game.current_phase().is_night() {
            self.night_defense(game)
        }else{
            self.role_state(game).clone().defense(game, *self)
        }
    }
    pub fn control_immune(&self, game: &Game) -> bool {
        self.role(game).control_immune()
    }
    pub fn end_game_condition(&self, game: &Game) -> EndGameCondition {
        self.role(game).end_game_condition()
    }
    pub fn has_innocent_aura(&self, game: &Game) -> bool {
        self.role(game).has_innocent_aura(game)
    }
    pub fn has_suspicious_aura(&self, game: &Game) -> bool {
        self.role(game).has_suspicious_aura(game) || 
        self.night_framed(game) || 
        (
            game.arsonist_doused().doused(*self) &&
            PlayerReference::all_players(game).any(|player_ref|
                player_ref.alive(game) && player_ref.role(game) == Role::Arsonist
            )
        )
    }

    /*
        Role functions
    */

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
    pub fn get_current_receive_chat_groups(&self, game: &Game) -> Vec<ChatGroup> {
        self.role_state(game).clone().get_current_receive_chat_groups(game, *self)
    }
    pub fn get_won_game(&self, game: &Game) -> bool {
        self.role_state(game).clone().get_won_game(game, *self)
    }
    pub fn convert_targets_to_visits(&self, game: &Game, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        self.role_state(game).clone().convert_targets_to_visits(game, *self, target_refs)
    }
    pub fn on_any_death(&self, game: &mut Game, dead_player_ref: PlayerReference){
        self.role_state(game).clone().on_any_death(game, *self, dead_player_ref)
    }
    pub fn on_game_ending(&self, game: &mut Game){
        self.role_state(game).clone().on_game_ending(game, *self)
    }
}



