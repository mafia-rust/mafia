use crate::game::{
    ability_input::AbilityInput,
    player::PlayerReference,
    Game
};

#[must_use = "Event must be invoked"]
pub struct OnValidatedAbilityInputReceived{
    actor_ref: PlayerReference,
    input: AbilityInput,
}
impl OnValidatedAbilityInputReceived{
    pub fn new(actor_ref: PlayerReference, input: AbilityInput) -> Self{
        Self{actor_ref, input}
    }
    pub fn invoke(self, game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_validated_ability_input_received(game, self.actor_ref, self.input.clone())
        }
    }
}