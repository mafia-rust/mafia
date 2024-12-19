use crate::game::{ability_input::{AbilityInput, ControllerID}, grave::GraveReference, role::RoleState, Game};

use super::PlayerReference;

impl PlayerReference {
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
}