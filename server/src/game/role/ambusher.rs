
use rand::seq::IndexedRandom;
use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
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
                let Some(visit) = actor_visits.first() else {return};
                let target_ref = visit.target;

                let player_to_attack = 
                if let Some(priority_visitor) = PlayerReference::all_players(game)
                    .filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref &&
                        other_player_ref.win_condition(game).is_loyalist_for(GameConclusion::Town) &&
                        // Its probably this way rather than having it filter applied directly to target_ref.all_night_visitors_cloned(game) 
                        // in order to prevent repeat players
                        target_ref.all_night_visitors_cloned(game).contains(other_player_ref)
                    ).collect::<Vec<PlayerReference>>()
                    .choose(&mut rand::rng())
                    .copied(){
                    Some(priority_visitor)
                } else {
                    PlayerReference::all_players(game)
                        .filter(|other_player_ref|
                            other_player_ref.alive(game) &&
                            *other_player_ref != actor_ref &&
                            // Its probably this way rather than having it filter applied directly to target_ref.all_night_visitors_cloned(game) 
                            // in order to prevent repeat players
                            target_ref.all_night_visitors_cloned(game).contains(other_player_ref)
                        )
                        .collect::<Vec<PlayerReference>>()
                        .choose(&mut rand::rng())
                        .copied()
                };

                if let Some(player_to_attack) = player_to_attack{
                    player_to_attack.try_night_kill_single_attacker(
                        actor_ref,
                        game,
                        GraveKiller::Role(Role::Ambusher),
                        AttackPower::Basic,
                        false
                    );

                    for visitor in target_ref.all_night_visitors_cloned(game){
                        if visitor == player_to_attack {continue;}
                        visitor.push_night_message(game, ChatMessageVariant::AmbusherCaught { ambusher: actor_ref });
                    }
                }
            }
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::builder(game)
            .id(ControllerID::role(actor_ref, Role::Ambusher, 0))
            .single_player_selection_typical(actor_ref, false, false)
            .night_typical(actor_ref)
            .add_grayed_out_condition(game.day_number() <= 1)
            .build_map()
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