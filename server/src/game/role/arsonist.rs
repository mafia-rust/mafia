use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::components::tags::{TagSetID, Tags};
use crate::game::grave::GraveKiller;
use crate::game::attack_power::DefensePower;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleStateImpl};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Arsonist;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Arsonist {
    type ClientRoleState = Arsonist;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Deception => {
                //douse target
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    let target_ref = visit.target;
                    Self::douse(game, target_ref);
                }
                
                //douse all visitors
                for other_player_ref in actor_ref.all_night_visitors_cloned(game)
                    .into_iter()
                    .filter(|other_player_ref| *other_player_ref != actor_ref)
                    .collect::<Vec<PlayerReference>>()
                {
                    Self::douse(game, other_player_ref);
                }
            },
            Priority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);             
                if let Some(visit) = actor_visits.first(){
                    if actor_ref == visit.target{
                        Self::ignite(game, actor_ref);
                    }
                }
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            true,
            true,
            false,
            ControllerID::role(actor_ref, Role::Arsonist, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Arsonist, 0),
            false
        )
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        Tags::remove_tag(game, TagSetID::ArsonistDoused, actor_ref);
        Tags::add_viewer(game, TagSetID::ArsonistDoused, actor_ref);
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, _new: super::RoleState, _old: super::RoleState) {
        if actor_ref == player {
            Tags::remove_viewer(game, TagSetID::ArsonistDoused, actor_ref); 
        }
    }
}
impl Arsonist{
    fn douse(game: &mut Game, player: PlayerReference){
        if player.role(game) == Role::Arsonist {
            return
        }

        Tags::add_tag(game, TagSetID::ArsonistDoused, player);
    }
    pub fn ignite(game: &mut Game, igniter: PlayerReference) {
        for player in Tags::tagged(game, TagSetID::ArsonistDoused) {
            if player.role(game) == Role::Arsonist {continue;}
            if !player.alive(game) {continue;}
            player.try_night_kill_single_attacker(
                igniter,
                game,
                GraveKiller::Role(Role::Arsonist),
                AttackPower::ProtectionPiercing,
                true
            );
        }
    }
    pub fn has_suspicious_aura_douse(game: &Game, player: PlayerReference) -> bool {
        Tags::has_tag(game, TagSetID::ArsonistDoused, player) &&
        PlayerReference::all_players(game).any(|player_ref|
            !player_ref.ability_deactivated_from_death(game) &&
            player_ref.role(game) == Role::Arsonist
        )
    }
}