use std::collections::{HashMap, HashSet};

use crate::game::
{
    chat::{ChatGroup, ChatMessageVariant}, components::{arsonist_doused::ArsonistDoused, puppeteer_marionette::PuppeteerMarionette}, event::{on_any_death::OnAnyDeath, on_role_switch::OnRoleSwitch}, resolution_state::ResolutionState, grave::{Grave, GraveKiller, GraveReference}, role::{same_evil_team, Priority, Role, RoleState}, visit::Visit, Game
};

use super::PlayerReference;

impl PlayerReference{
    pub fn roleblock(&self, game: &mut Game, send_messages: bool) {
        let roleblock_immune = self.role(game).roleblock_immune();

        if !roleblock_immune {
            self.set_night_roleblocked(game, true);
            self.set_night_visits(game, vec![]);
        }

        if send_messages {
            self.push_night_message(game,
                ChatMessageVariant::RoleBlocked { immune: roleblock_immune }
            );
        }
    }
    pub fn ward(&self, game: &mut Game) -> Vec<PlayerReference> {
        let mut wardblocked = vec![];
        for visitor in self.all_visitors(game){
            if !visitor.role(game).wardblock_immune() {
                visitor.set_night_wardblocked(game, true);
                visitor.set_night_visits(game, vec![]);
                visitor.push_night_message(game, ChatMessageVariant::Wardblocked);
                wardblocked.push(visitor);
            }
        }
        wardblocked
    }

    /// Returns true if attack overpowered defense
    pub fn try_night_kill(&self, attacker_ref: PlayerReference, game: &mut Game, grave_killer: GraveKiller, attack: u8, should_leave_death_note: bool) -> bool {
        self.set_night_attacked(game, true);

        if self.night_defense(game) >= attack {
            self.push_night_message(game, ChatMessageVariant::YouSurvivedAttack);
            attacker_ref.push_night_message(game,ChatMessageVariant::SomeoneSurvivedYourAttack);
            return false;
        }
        
        self.push_night_message(game, ChatMessageVariant::YouWereAttacked);
        attacker_ref.push_night_message(game,ChatMessageVariant::YouAttackedSomeone);

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
    pub fn try_night_kill_anonymous(&self, game: &mut Game, grave_killer: GraveKiller, attack: u8) -> bool {
        self.set_night_attacked(game, true);

        if self.night_defense(game) >= attack {
            self.push_night_message(game, ChatMessageVariant::YouSurvivedAttack);
            return false;
        }
        
        self.push_night_message(game, ChatMessageVariant::YouWereAttacked);

        self.push_night_grave_killers(game, grave_killer);        

        if !self.alive(game) { return true }

        self.set_night_died(game, true);

        true
    }

    /**
    ### Example use in minion case
        
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if let Some(currently_used_player) = actor_ref.possess_night_action(game, priority, self.currently_used_player){
            actor_ref.set_role_state(game, RoleState::Minion(Minion{
                currently_used_player: Some(currently_used_player)
            }))
        }
    }
    */
    pub fn possess_night_action(&self, game: &mut Game, priority: Priority, currently_used_player: Option<PlayerReference>)->Option<PlayerReference>{
        match priority {
            Priority::Possess => {
                let possessor_visits = self.night_visits(game).clone();
                let Some(possessed_visit) = possessor_visits.get(0) else {return None};
                let Some(possessed_into_visit) = possessor_visits.get(1) else {return None};
                
                possessed_visit.target.push_night_message(game,
                    ChatMessageVariant::YouWerePossessed { immune: possessed_visit.target.possession_immune(game) }
                );
                if possessed_visit.target.possession_immune(game) {
                    self.push_night_message(game,
                        ChatMessageVariant::TargetIsPossessionImmune
                    );
                    return None;
                }

                let mut new_selection = possessed_visit.target
                    .night_visits(game)
                    .iter()
                    .map(|v|v.target)
                    .collect::<Vec<PlayerReference>>();

                if let Some(target) = new_selection.first_mut(){
                    *target = possessed_into_visit.target;
                }else{
                    new_selection = vec![possessed_into_visit.target];
                }

                possessed_visit.target.set_night_visits(game,
                    possessed_visit.target.convert_selection_to_visits(game, new_selection)
                );

                self.set_night_visits(game, vec![possessed_visit.clone()]);
                return Some(possessed_visit.target);
            },
            Priority::Investigative => {
                if let Some(currently_used_player) = currently_used_player {
                    self.push_night_message(game,
                        ChatMessageVariant::PossessionTargetsRole { role: currently_used_player.role(game) }
                    );
                }
                return None;
            },
            Priority::StealMessages => {
                if let Some(currently_used_player) = currently_used_player {
                    for message in currently_used_player.night_messages(game).clone() {
                        self.push_night_message(game,
                            ChatMessageVariant::TargetsMessage { message: Box::new(message.clone()) }
                        );
                    }
                }
                return None;
            },
            _ => {
                return None;
            }
        }
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

        OnAnyDeath::new(*self)
    }
    /// Swaps this persons role, sends them the role chat message, and makes associated changes
    pub fn set_role(&self, game: &mut Game, new_role_data: RoleState){

        let old: Role = self.role(game);

        self.set_role_state(game, new_role_data.clone());
        self.on_role_creation(game);
        if new_role_data.role() == self.role(game) {
            self.add_private_chat_message(game, ChatMessageVariant::RoleAssignment{role: self.role(game)});
        }

        OnRoleSwitch::new(*self, old, self.role(game)).invoke(game);
    }
    pub fn increase_defense_to(&self, game: &mut Game, defense: u8){
        if self.night_defense(game) < defense {
            self.set_night_upgraded_defense(game, Some(defense));
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
        for other in PlayerReference::all_players(game){
            if *self == other { continue }
            

            if same_evil_team(game, *self, other) {
                other.insert_role_label(game, *self);
                self.insert_role_label(game, other);
            }
        }
    }

    pub fn role_label_map(&self, game: &Game) -> HashMap<PlayerReference, Role> {
        let mut map = HashMap::new();
        for player in self.role_labels(game) {
            map.insert(*player, player.role(game));
        }
        map
    }

    pub fn defense(&self, game: &Game) -> u8 {
        if game.current_phase().is_night() {
            self.night_defense(game)
        }else{
            self.role(game).defense()
        }
    }
    pub fn possession_immune(&self, game: &Game) -> bool {
        self.role(game).possession_immune()
    }
    pub fn has_innocent_aura(&self, game: &Game) -> bool {
        self.role(game).has_innocent_aura(game)
    }
    pub fn has_suspicious_aura(&self, game: &Game) -> bool {
        self.role(game).has_suspicious_aura(game) || 
        self.night_framed(game) ||
        ArsonistDoused::has_suspicious_aura_douse(game, *self)
    }
    pub fn required_resolution_states_for_win(&self, game: &Game) -> Option<HashSet<ResolutionState>> {
        if PuppeteerMarionette::is_marionette(game, *self) {return Some(vec![ResolutionState::Fiends].into_iter().collect());}
        ResolutionState::required_resolution_states_for_win(self.role(game))
    }
    pub fn keeps_game_running(&self, game: &Game) -> bool {
        if PuppeteerMarionette::is_marionette(game, *self) {return false;}
        ResolutionState::keeps_game_running(self.role(game))
    }

    /*
        Role functions
    */

    pub fn can_select(&self, game: &Game, target_ref: PlayerReference) -> bool {
        self.role_state(game).clone().can_select(game, *self, target_ref)
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
    pub fn convert_selection_to_visits(&self, game: &Game, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        self.role_state(game).clone().convert_selection_to_visits(game, *self, target_refs)
    }
    pub fn on_any_death(&self, game: &mut Game, dead_player_ref: PlayerReference){
        self.role_state(game).clone().on_any_death(game, *self, dead_player_ref)
    }
    pub fn on_grave_added(&self, game: &mut Game, grave: GraveReference){
        self.role_state(game).clone().on_grave_added(game, *self, grave)
    }
    pub fn on_game_ending(&self, game: &mut Game){
        self.role_state(game).clone().on_game_ending(game, *self)
    }
}



