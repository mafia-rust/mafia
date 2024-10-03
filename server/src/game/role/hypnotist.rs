use serde::{Deserialize, Serialize};

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::visit::Visit;
use crate::game::Game;
use super::{Priority, RoleState, RoleStateImpl};

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

    pub target: Option<PlayerReference>,
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
            
            target: None,
        }
    }
}
pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    pub roleblock: bool,
    pub you_were_roleblocked_message: bool,
    pub you_survived_attack_message: bool,
    pub you_were_protected_message: bool,
    pub you_were_transported_message: bool,
    pub you_were_possessed_message: bool,
    pub your_target_was_jailed_message: bool,

    pub target: Option<PlayerReference>,
}

impl RoleStateImpl for Hypnotist {
    type ClientRoleState = Hypnotist;
    type RoleActionChoice = RoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        let Some(visit) = actor_ref.night_visits(game).first() else {
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
    fn on_role_action(self, game: &mut Game, actor_ref: PlayerReference, mut action_choice: Self::RoleActionChoice) {
        RoleActionChoice::ensure_at_least_one_message(&mut action_choice);
        crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.target, false);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(self.target, false)
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
impl RoleActionChoice{
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