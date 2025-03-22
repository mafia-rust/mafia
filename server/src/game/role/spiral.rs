use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::attack_type::AttackData;
use crate::game::components::poison::{Poison, PoisonAlert};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;

use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set::VecSet;

use super::{common_role, ControllerID, GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

#[derive(Debug, Clone, Default)]
pub struct Spiral{
    pub spiraling: VecSet<PlayerReference>
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Spiral {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        let mut new_spiraling = VecSet::new();

        if priority != Priority::Poison { return };
        
        if self.spiraling.is_empty() && game.day_number() > 1 {
            if let Some(visit) = actor_ref.untagged_night_visits_cloned(game).first(){
                let target_ref = visit.target;
                
                target_ref.try_night_kill_single_attacker(
                    actor_ref,
                    game,
                    GraveKiller::Role(Role::Spiral),
                    AttackPower::ArmorPiercing,
                    true
                );
                Spiral::spiral_visitors(game, &mut new_spiraling, actor_ref, target_ref);
            }
        } else {
            for spiraling_player in self.spiraling.clone() {
                Spiral::remove_player_spiraling(game, &mut new_spiraling, actor_ref, spiraling_player);

                Spiral::spiral_visitors(game, &mut new_spiraling, actor_ref, spiraling_player);
            }
        }

        actor_ref.set_role_state(game, RoleState::Spiral(Spiral{spiraling: new_spiraling}));
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            game.day_number() <= 1 || !self.spiraling.is_empty(),
            ControllerID::role(actor_ref, Role::Spiral, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Spiral, 0),
            true
        )
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, _old: RoleState, _new: RoleState) {
        if player == actor_ref {
            actor_ref.remove_player_tag_on_all(game, Tag::Spiraling);
        }
    }
    fn attack_data(&self, game: &Game, actor_ref: PlayerReference) -> AttackData {
        AttackData::attack(game, actor_ref, false)
    }
}

impl Spiral {
    fn start_player_spiraling(game: &mut Game, new_spiraling: &mut VecSet<PlayerReference>, actor_ref: PlayerReference, target_ref: PlayerReference) {
        let attackers = vec![actor_ref].into_iter().collect();
        if target_ref == actor_ref {
            return;
        }
        Poison::poison_player(game, target_ref, 
            AttackPower::ArmorPiercing, 
            GraveKiller::Role(Role::Spiral), 
            attackers, 
            true, 
            PoisonAlert::NoAlert,
        );

        new_spiraling.insert(target_ref);
        actor_ref.push_player_tag(game, target_ref, Tag::Spiraling);
    }

    fn remove_player_spiraling(game: &mut Game, new_spiraling: &mut VecSet<PlayerReference>, actor_ref: PlayerReference, target_ref: PlayerReference) {
        new_spiraling.remove(&target_ref);
        actor_ref.remove_player_tag(game, target_ref, Tag::Spiraling);
    }

    fn spiral_visitors(game: &mut Game, new_spiraling: &mut VecSet<PlayerReference>, actor_ref: PlayerReference, target: PlayerReference) {
        for visitor_to_spiraling in target.all_night_visitors_cloned(game)
            .into_iter().filter(|other_player_ref|
                other_player_ref.alive(game) &&
                *other_player_ref != target // Let doctor self-heal
            ).collect::<Vec<PlayerReference>>()
        {
            Spiral::start_player_spiraling(game, new_spiraling, actor_ref, visitor_to_spiraling);
        }
    }
}

impl GetClientRoleState<ClientRoleState> for Spiral {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}