
use rand::seq::IndexedRandom;
use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::night_visits::NightVisits;
use crate::game::game_conclusion::GameConclusion;
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{
    ControllerID, ControllerParametersMap,
    Priority, Role, RoleStateImpl
};



#[derive(Clone, Debug, Default, Serialize)]
pub struct Ambusher;


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Ambusher {
    type ClientRoleState = Ambusher;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() <= 1 {return}

        match priority {
            Priority::Kill => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(ambush_visit) = actor_visits.first() else {return};
                let target_ref = ambush_visit.target;

                let player_to_attacks_visit = 
                if let Some(priority_visitor) = NightVisits::all_visits(game).into_iter()
                    .filter(|visit|
                        ambush_visit != *visit &&
                        visit.target == target_ref &&
                        visit.visitor.alive(game) &&
                        visit.visitor.win_condition(game).is_loyalist_for(GameConclusion::Town)
                    ).collect::<Vec<&Visit>>()
                    .choose(&mut rand::rng())
                    .copied()
                {
                    Some(priority_visitor.visitor)
                } else {
                    NightVisits::all_visits(game).into_iter()
                        .filter(|visit|
                            ambush_visit != *visit &&
                            visit.target == target_ref &&
                            visit.visitor.alive(game)
                        ).collect::<Vec<&Visit>>()
                        .choose(&mut rand::rng())
                        .copied()
                        .map(|v|v.visitor)
                };

                if let Some(player_to_attack) = player_to_attacks_visit{
                    player_to_attack.try_night_kill_single_attacker(
                        actor_ref,
                        game,
                        GraveKiller::Role(Role::Ambusher),
                        AttackPower::Basic,
                        false
                    );

                    for visitor in target_ref.all_night_visitors_cloned(game){
                        if visitor == player_to_attack || visitor == actor_ref {continue;}
                        visitor.push_night_message(game, ChatMessageVariant::AmbusherCaught { ambusher: actor_ref });
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
            false,
            false,
            !(game.day_number() > 1),
            ControllerID::role(actor_ref, Role::Ambusher, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Ambusher, 0),
            false
        )
    }
    fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
       vec![
           crate::game::components::insider_group::InsiderGroupID::Mafia
       ].into_iter().collect()
   }
}