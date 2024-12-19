use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;


use crate::game::visit::Visit;
use crate::game::Game;
use super::{ControllerID, ControllerParametersMap, Priority, Role, RoleState, RoleStateImpl};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hypnotist{
    pub roleblock: bool,
    pub you_were_roleblocked_message: bool,
    pub you_survived_attack_message: bool,
    pub you_were_protected_message: bool,
    pub you_were_transported_message: bool,
    pub you_were_possessed_message: bool,
    pub your_target_was_jailed_message: bool,
}


impl Default for Hypnotist {
    fn default() -> Self {
        Self {
            roleblock: true,
            you_were_roleblocked_message: true,
            you_survived_attack_message: false,
            you_were_protected_message: false,
            you_were_transported_message: false,
            you_were_possessed_message: false,
            your_target_was_jailed_message: false,
        }
    }
}

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Hypnotist {
    type ClientRoleState = Hypnotist;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        let Some(visit) = actor_visits.first() else {
            return;
        };
        let target_ref = visit.target;
        

        match priority {
            Priority::TopPriority => {
                let mut hypnotist = self.clone();
                hypnotist.ensure_at_least_one_message();
                actor_ref.set_role_state(game, RoleState::Hypnotist(self));
            },
            Priority::Roleblock => {
                if self.roleblock {
                    target_ref.roleblock(game, false);
                }
            },
            Priority::Deception => {
                if self.you_were_roleblocked_message {
                    if target_ref.role(game).roleblock_immune() {
                        target_ref.push_night_message(game, ChatMessageVariant::RoleBlocked { immune: true });
                    } else {
                        target_ref.push_night_message(game, ChatMessageVariant::RoleBlocked { immune: false });
                    }
                }
                if self.you_survived_attack_message {
                    target_ref.push_night_message(game, ChatMessageVariant::YouSurvivedAttack);
                }
                if self.you_were_protected_message {
                    target_ref.push_night_message(game, ChatMessageVariant::YouWereProtected);
                }
                if self.you_were_transported_message {
                    target_ref.push_night_message(game, ChatMessageVariant::Transported);
                }
                if self.you_were_possessed_message {
                    if target_ref.role(game).possession_immune() {
                        target_ref.push_night_message(game, ChatMessageVariant::YouWerePossessed { immune: true });
                    } else {
                        target_ref.push_night_message(game, ChatMessageVariant::YouWerePossessed { immune: false });
                    }
                }
                if self.your_target_was_jailed_message {
                    target_ref.push_night_message(game, ChatMessageVariant::Wardblocked);
                }
            },
            _ => {}
        }
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        crate::game::role::common_role::controller_parameters_map_player_list_night_typical(
            game,
            actor_ref,
            false,
            false,
            ControllerID::role(actor_ref, Role::Hypnotist, 0)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Hypnotist, 0),
            false
        )
    }
     fn default_revealed_groups(self) -> crate::vec_set::VecSet<crate::game::components::insider_group::InsiderGroupID> {
        vec![
            crate::game::components::insider_group::InsiderGroupID::Mafia
        ].into_iter().collect()
    }
}
impl Hypnotist {
    pub fn ensure_at_least_one_message(&mut self){
        if
            !self.you_were_roleblocked_message && 
            !self.you_survived_attack_message && 
            !self.you_were_protected_message && 
            !self.you_were_transported_message && 
            !self.you_were_possessed_message && 
            !self.your_target_was_jailed_message
        {
            self.you_were_roleblocked_message = true;
        }
    }
}