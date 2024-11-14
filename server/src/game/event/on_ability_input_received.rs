use crate::game::{
    ability_input::AbilityInput, components::{forfeit_vote::ForfeitVote, generic_ability::GenericAbilitySaveComponent, pitchfork::Pitchfork}, modifiers::Modifiers, player::PlayerReference, Game
};

#[must_use = "Event must be invoked"]
pub struct OnAbilityInputReceived{
    actor_ref: PlayerReference,
    input: AbilityInput,
}
impl OnAbilityInputReceived{
    pub fn new(actor_ref: PlayerReference, input: AbilityInput) -> Self{
        Self{actor_ref, input}
    }
    pub fn invoke(self, game: &mut Game){
        for player_ref in PlayerReference::all_players(game){
            player_ref.on_ability_input_received(game, self.actor_ref, self.input.clone())
        }
        Pitchfork::on_ability_input_received(game, self.actor_ref, self.input.clone());
        Modifiers::on_ability_input_received(game, self.actor_ref, self.input.clone());
        ForfeitVote::on_ability_input_received(game, self.actor_ref, self.input.clone());
        GenericAbilitySaveComponent::on_ability_input_received(game, self.actor_ref, self.input);
    }
}