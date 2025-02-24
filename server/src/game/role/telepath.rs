use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::{attack_power::DefensePower, grave::GraveKiller};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{common_role, Priority, Role, RoleStateImpl};
use crate::game::ability_input::*;
type Message = ChatMessageVariant;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Telepath;

pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Telepath {
    type ClientRoleState = Telepath;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Roleblock => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let is_wardblock_message = game.saved_controllers
                    .get_controller_current_selection_boolean(
                        ControllerID::role(actor_ref, Role::Warden, 1)
                    )
                    .is_some_and(|is_wardblock_message: BooleanSelection|is_wardblock_message.0);
                for visit in actor_visits.iter() {
                    let target_ref = visit.target;
                    target_ref.roleblock(game, false);
                    match (is_wardblock_message, target_ref.role(&game).wardblock_immune(), target_ref.role(&game).roleblock_immune()) {
                        (true,  true, immune) => target_ref.push_night_message(game, Message::RoleBlocked {immune}),
                        (true, false, _) => target_ref.push_night_message(game, Message::Wardblocked),
                        (false, _, immune) => target_ref.push_night_message(game, Message::RoleBlocked {immune}),
                    }
                }
            },
            Priority::Kill => {
                if game.day_number() == 1 {return}
                
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                for visit in actor_visits.iter(){
                    if !visit.attack {continue;}
                    let target_ref = visit.target;
                    
                    target_ref.try_night_kill_single_attacker(
                        actor_ref,
                        game,
                        GraveKiller::Role(Role::Telepath),
                        AttackPower::ArmorPiercing,
                        true
                    );
                }
            },
            Priority::StealMessages => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);

                for visit in actor_visits.iter(){
                    actor_ref.push_night_message(game, 
                        Message::ReceivedMessagesStart { recipient: visit.target }
                    );
                    for message in visit.target.night_messages(game).clone() {
                        actor_ref.push_night_message(game,
                            Message::TargetsMessage { message: Box::new(message.clone()) }
                        );
                    }
                    actor_ref.push_night_message(game, Message::ReceivedMessagesEnd);
                }
            },
            _ => (),
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        let mut controllers = common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            game.day_number() <= 1,
            ControllerID::role(actor_ref, Role::Telepath, 0)
        );
        controllers.combine_overwrite(common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            true,
            false,
            ControllerID::role(actor_ref, Role::Telepath, 1)
        ));
        controllers.combine_overwrite(common_role::controller_parameters_map_boolean(
            game, 
            actor_ref, 
            false,
            ControllerID::role(actor_ref, Role::Telepath, 2)
        ));
        return controllers;
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let mut visits = crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Telepath, 0),
            true
        );
        visits.append(
            &mut crate::game::role::common_role::convert_controller_selection_to_visits(
                game,
                actor_ref,
                ControllerID::role(actor_ref, Role::Telepath, 1),
                false
            )
        );
        for visit in visits.iter() {
            println!("visit target: {}", visit.target.index());
            println!("visit attack: {}", visit.attack);
        }
        return visits;
    }
}
