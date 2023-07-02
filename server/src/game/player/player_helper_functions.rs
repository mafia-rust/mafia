use crate::{game::{chat::{ChatMessage, ChatGroup}, Game, grave::{GraveKiller, Grave}, role::{RoleState, Priority}, visit::Visit, team::{Teams, Team}, end_game_condition::EndGameCondition}, packet::ToClientPacket};

use super::PlayerReference;



impl PlayerReference{
    ///returns true if they are not roleblockable
    pub fn roleblock(&self, game: &mut Game)->bool{
        if !self.role_state(game).roleblock_immune(game, *self) {
            self.set_night_roleblocked(game, true);
            self.set_night_visits(game, vec![]);
            self.push_night_message(game,
                ChatMessage::RoleBlocked { immune: false }
            );
            true
        } else {
            self.push_night_message(game,
                ChatMessage::RoleBlocked { immune: true }
            );
            false
        }
    }
    ///returns true if attack overpowered defense.
    pub fn try_night_kill(&self, attacker_ref: PlayerReference, game: &mut Game, grave_killer: GraveKiller, attack: u8)->bool{
        self.set_night_attacked(game, true);

        if self.night_defense(game) >= attack {
            self.push_night_message(game,
                ChatMessage::YouSurvivedAttack
            );
            attacker_ref.push_night_message(game,ChatMessage::TargetSurvivedAttack);
            return false;
        }

        if !self.alive(game){
            return true;
        }
        self.set_night_died(game, true);
        // self.set_alive(game, false); TODO remove this comment if it doesnt cause problems. 6/14/2023
        self.push_night_grave_killers(game, grave_killer);

        true
    }
    pub fn die(&self, game: &mut Game, grave: Grave){
        self.set_alive(game, false);

        self.add_chat_message(game, ChatMessage::YouDied);
        game.graves.push(grave.clone());
        game.send_packet_to_all(ToClientPacket::AddGrave{grave: grave.clone()});
        game.add_message_to_chat_group(ChatGroup::All, ChatMessage::PlayerDied { grave: grave.clone() });

        if let Some(role) = grave.role.get_role(){
            for other_player_ref in PlayerReference::all_players(game){
                other_player_ref.insert_role_label(game, *self, role);
            }
        }

        for player_ref in PlayerReference::all_players(game){
            player_ref.on_any_death(game, *self)
        }
        Teams::on_any_death(game);
    }
    /// swap this persons role, sending them the role chat message, and associated changes
    pub fn set_role(&self, game: &mut Game, new_role_data: RoleState){

        self.set_role_state(game, new_role_data);
        self.on_role_creation(game);
        self.add_chat_message(game, ChatMessage::RoleAssignment{role: self.role(game)});

        self.insert_role_label(game, *self, self.role(game));
        if let Some(team) = self.team(game) {

            team.team_state(&game.teams).on_member_role_switch(game, *self);

            for player in team.members(game) {
                player.insert_role_label(game, *self, self.role(game));
                self.insert_role_label(game, player, player.role(game));
            }
        }
    }
    pub fn increase_defense_to(&self, game: &mut Game, defense: u8){
        if self.night_defense(game) < defense {
            self.set_night_defense(game, defense);
        }
    }
    
    pub fn tracker_seen_visits(self, game: &Game) -> Vec<&Visit> {
        if let Some(v) = self.night_appeared_visits(game) {
            v.iter().filter(|v|!v.astral).collect()
        } else {
            self.night_visits(game).iter().filter(|v|!v.astral).collect()
        }
    }
    ///includes self obviously
    pub fn lookout_seen_players(self, game: &Game) -> Vec<PlayerReference> {
        PlayerReference::all_players(game).into_iter().filter(|player_ref|{
            player_ref.tracker_seen_visits(game).iter().any(|other_visit| 
                other_visit.target == self && !other_visit.astral
            )
        }).collect()
    }



    pub fn insert_role_label_for_teammates(&self, game: &mut Game){
        let actor_role = self.role(game);
    
    
        //if they are on a team. set labels for their teammates, and my label for my teammates
        for other in PlayerReference::all_players(game){
            if *self == other{
                continue;
            }
            let other_role = other.role(game);
            
            if Team::same_team(game, *self, other) {
                other.insert_role_label(game, *self, actor_role);
                self.insert_role_label(game, other, other_role);
            }
        }
    }




    //role functions
    pub fn suspicious(&self, game: &Game) -> bool {
        self.role_state(game).suspicious(game, *self)
    }
    pub fn defense(&self, game: &Game) -> u8 {
        self.role_state(game).defense(game, *self)
    }
    pub fn control_immune(&self, game: &Game) -> bool {
        self.role_state(game).control_immune(game, *self)
    }
    pub fn roleblock_immune(&self, game: &Game) -> bool {
        self.role_state(game).roleblock_immune(game, *self)
    }
    pub fn end_game_condition(&self, game: &Game) -> EndGameCondition {
        self.role_state(game).end_game_condition(game, *self)
    }
    pub fn team(&self, game: &Game) -> Option<Team> {
        self.role_state(game).team(game, *self)
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
    pub fn on_any_death(&self, game: &mut Game, dead_player_ref: PlayerReference){
        self.role_state(game).clone().on_any_death(game, *self, dead_player_ref)
    }
}



