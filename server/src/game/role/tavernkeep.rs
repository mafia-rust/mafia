use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::components::confused::Confused;
use crate::game::components::drunk_aura::DrunkAura;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, Role, RoleStateImpl};
use crate::game::ability_input::*;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Tavernkeep;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Tavernkeep {
    type ClientRoleState = Tavernkeep;
    fn before_initial_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        DrunkAura::add_player_permanent(game, actor_ref);
    }
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::ConfusePossessors => {
                //Confuses players that try to possess the Tavernkeep
                //Currently doesn't have any effect because players that can possess can't possess multiple players and are 
                for possessor_visiting_tavernkeep in actor_ref.all_night_visitors_cloned(game)
                    .into_iter()
                    .collect::<Vec<PlayerReference>>()
                {
                    if possessor_visiting_tavernkeep.role(game).can_possess() {
                        Confused::add_player(game, possessor_visiting_tavernkeep, 0);
                    }
                }

                //Confuses the targets of the Tavernkeep that can possess players
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);

                for visit in actor_visits {
                    if visit.target.role(game).can_possess() {
                        Confused::add_player(game, visit.target, 0);
                    }
                }
            }
            Priority::Confuse => {
                //Confuses and gives drunk aura to players visiting the Tavernkeep
                for player_visiting_tavernkeep in actor_ref.all_night_visitors_cloned(game)
                    .into_iter()
                    .filter(|other_player_ref| *other_player_ref != actor_ref)
                    .collect::<Vec<PlayerReference>>()
                {   
                    Confused::add_player(game, player_visiting_tavernkeep, 1);
                    DrunkAura::add_player(game, player_visiting_tavernkeep, 1);
                }

                //Confuses and gives drunk aura to the targets of the Tavernkeep
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);

                for visit in actor_visits {
                    Confused::add_player(game, visit.target, 1);
                    DrunkAura::add_player(game, visit.target, 1);
                    if visit.attack {
                        visit.target.try_night_kill_single_attacker(
                            actor_ref,
                            game,
                            GraveKiller::Role(Role::Tavernkeep),
                            AttackPower::ArmorPiercing,
                            true
                        );
                    }
                }
            }
            Priority::Kill => {
                if game.day_number() == 1 {return}
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                if let Some(visit) = actor_visits.first(){
                    if !visit.attack {return;}

                    let target_ref = visit.target;
                    
                    target_ref.try_night_kill_single_attacker_ignore_confusion(
                        actor_ref,
                        game,
                        GraveKiller::Role(Role::Tavernkeep),
                        AttackPower::ArmorPiercing,
                        true
                    );
                }
            }
            
            _=> (),
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        //make drunk
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Tavernkeep, 0)
        );
        //attack
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Tavernkeep, 1)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let mut visits = crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Tavernkeep, 0),
            true
        );
        // No visit for the attack because idk how to differentiate the visit for confusion & for attack
        // without breaking if there is a transporter/warper.

        visits.append(& mut crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Tavernkeep, 1),
            false
        ));
        return visits;
    }
}
