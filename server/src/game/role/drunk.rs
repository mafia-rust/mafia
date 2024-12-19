use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

use crate::game::components::drunk_aura::DrunkAura;
use crate::game::{attack_power::DefensePower, components::confused::Confused};
use crate::game::player::PlayerReference;
use crate::game::Game;

use super::{Role, RoleStateImpl};


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Drunk;

impl RoleStateImpl for Drunk {
    type ClientRoleState = Drunk;
    fn before_initial_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {

        let possible_roles = Self::POSSIBLE_ROLES.iter()
            .filter(|role|game.settings.enabled_roles.contains(role))
            .collect::<Vec<_>>();

        //special case here. I don't want to use set_role because it alerts the player their role changed
        //NOTE: It will still send a packet to the player that their role state updated,
        //so it might be deducable that there is a recruiter
        if let Some(random_town_role) = possible_roles.choose(&mut thread_rng()) {
            actor_ref.set_role_state(game, random_town_role.new_state(game));
        }

        Confused::add_player(game, actor_ref);
        DrunkAura::add_player(game, actor_ref);
    }
}
impl Drunk{
    const POSSIBLE_ROLES: [Role; 7] = [
        Role::Detective, Role::Snoop, Role::Gossip,
        Role::Philosopher, Role::Psychic, Role::TallyClerk,
        Role::Auditor
    ];
}