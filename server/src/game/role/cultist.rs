use serde::Serialize;
use crate::game::components::insider_group::InsiderGroupID;
use crate::vec_set;
use crate::{game::{attack_power::DefensePower, components::cult::Cult, player::PlayerReference, Game}, vec_set::VecSet};


use super::{disciple::Disciple, GetClientRoleState, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Cultist;

pub type ClientRoleState = Cultist;

impl RoleStateImpl for Cultist {
    type ClientRoleState = Disciple;
    fn default_revealed_groups(self) -> VecSet<InsiderGroupID> {
        vec_set![InsiderGroupID::Cult]
    }
    fn on_role_creation(self, game: &mut Game, _actor_ref: PlayerReference) {
        Cult::set_ordered_cultists(game);
    }
}

impl GetClientRoleState<Disciple> for Cultist {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> Disciple {
        Disciple
    }
}