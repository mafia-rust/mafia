
use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::Game;
use super::{GetClientRoleState, RoleState, RoleStateImpl};

#[derive(Clone, Debug, Default)]
pub struct Mayor {
    pub revealed: bool
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Mayor { 
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = ();
    fn on_role_action(self, game: &mut Game, actor_ref: PlayerReference, _action_choice: Self::RoleActionChoice) {
        if !(
            game.current_phase().is_day() &&
            actor_ref.alive(game) &&
            !self.revealed
        ){
            return;
        }

        game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MayorRevealed { player_index: actor_ref.index() });

        actor_ref.set_role_state(game, RoleState::Mayor(Mayor{
            revealed: true
        }));
        for player in PlayerReference::all_players(game){
            player.insert_role_label(game, actor_ref);
        }
        game.count_votes_and_start_trial();
    }
}
impl GetClientRoleState<ClientRoleState> for Mayor {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}