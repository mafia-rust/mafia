use crate::game::{ability_input::{AbilityInput, ControllerID}, event::{on_midnight::{MidnightVariables, OnMidnight, OnMidnightPriority}, on_whisper::{OnWhisper, WhisperFold, WhisperPriority}}, grave::GraveReference, role::RoleState, visit::Visit, Game};

use super::PlayerReference;

impl PlayerReference {

    pub fn on_midnight(game: &mut Game, _event: &OnMidnight, midnight_variables: &mut MidnightVariables, priority: OnMidnightPriority){
        if priority == OnMidnightPriority::InitializeNight {
            for player_ref in PlayerReference::all_players(game){
                player_ref.set_night_grave_will(midnight_variables, player_ref.will(game).clone());
            }

            for player_ref in PlayerReference::all_players(game){
                let visits = player_ref.convert_selection_to_visits(game);
                player_ref.set_night_visits(game, visits.clone());
            }
        }

        for player_ref in PlayerReference::all_players(game){
            player_ref.on_midnight_one_player(game, midnight_variables, priority);
        }

        if priority == OnMidnightPriority::FinalizeNight {
            for player_ref in PlayerReference::all_players(game){
                player_ref.push_night_messages_to_player(game, midnight_variables);
            }
        }
    }



    pub fn on_controller_selection_changed(&self, game: &mut Game, id: ControllerID){
        self.role_state(game).clone().on_controller_selection_changed(game, *self, id)
    }
    pub fn on_validated_ability_input_received(&self, game: &mut Game, input_player: PlayerReference, input: AbilityInput) {
        self.role_state(game).clone().on_validated_ability_input_received(game, *self, input_player, input)
    }
    pub fn on_ability_input_received(&self, game: &mut Game, input_player: PlayerReference, input: AbilityInput) {
        self.role_state(game).clone().on_ability_input_received(game, *self, input_player, input)
    }
    pub fn on_game_start(&self, game: &mut Game){
        self.role_state(game).clone().on_game_start(game, *self)
    }
    pub fn on_game_ending(&self, game: &mut Game){
        self.role_state(game).clone().on_game_ending(game, *self)
    }
    pub fn on_grave_added(&self, game: &mut Game, grave: GraveReference){
        self.role_state(game).clone().on_grave_added(game, *self, grave)
    }
    pub fn on_any_death(&self, game: &mut Game, dead_player_ref: PlayerReference){
        self.role_state(game).clone().on_any_death(game, *self, dead_player_ref)
    }
    pub fn before_role_switch(&self, game: &mut Game, player: PlayerReference, old: RoleState, new: RoleState,){
        self.role_state(game).clone().before_role_switch(game, *self, player, old, new);
    }
    pub fn before_initial_role_creation(&self, game: &mut Game){
        self.role_state(game).clone().before_initial_role_creation(game, *self)
    }
    pub fn on_player_roleblocked(&self, game: &mut Game, player: PlayerReference, invisible: bool) {
        self.role_state(game).clone().on_player_roleblocked(game, *self, player, invisible)
    }
    pub fn on_visit_wardblocked(&self, game: &mut Game, visit: Visit) {
        self.role_state(game).clone().on_visit_wardblocked(game, *self, visit)
    }
    pub fn on_whisper(game: &mut Game, event: &OnWhisper, fold: &mut WhisperFold, priority: WhisperPriority) {
        for player in PlayerReference::all_players(game){
            player.role_state(game).clone().on_whisper(game, player, event, fold, priority);
        }
    }
}