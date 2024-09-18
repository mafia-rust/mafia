
use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::Game;
use super::{RoleStateImpl, RoleState};

#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mayor {
    pub revealed: bool
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Mayor {
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, _target_ref: PlayerReference) {

        if !actor_ref.alive(game) || !game.current_phase().is_day() {
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
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool{
        game.current_phase().is_day() &&
        !self.revealed &&
        actor_ref == target_ref &&
        actor_ref.alive(game) &&
        PhaseType::Night != game.current_phase().phase()
    }
}