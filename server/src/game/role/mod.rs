use crate::game::player::PlayerReference;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::chat::ChatGroup;
use crate::game::role_list::FactionAlignment;
use crate::game::phase::PhaseType;
use crate::game::team::Team;

use serde::{Serialize, Deserialize};

trait RoleStateImpl: Clone + std::fmt::Debug + Serialize + Default {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority);
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference);

    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool;
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool;

    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit>;

    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>;
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>;

    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType);
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference);
}

// Creates the Role enum
macros::roles! {
    Jailor : jailor,

    Sheriff : sheriff,
    Lookout : lookout,

    Doctor : doctor,
    Bodyguard : bodyguard,

    Vigilante : vigilante,
    Veteran : veteran,

    Escort : escort,
    Medium : medium,
    Retributionist : retributionist,

    //Mafia
    Mafioso : mafioso,
    
    Consort : consort,
    Blackmailer : blackmailer,

    Janitor : janitor,
    Framer : framer,

    //Coven
    CovenLeader : coven_leader,

    VoodooMaster : voodoo_master,

    Jester : jester
}

macros::priorities! {
    TopPriority,
    Unswappable,
    Transporter,
    Control,
    Necromancy,

    Roleblock,
    Deception,

    Trickster,
    Bodyguard,

    Heal,
    Kill,
    Investigative,
    StealMessages
}

mod common_role;

mod macros {
    macro_rules! roles {
        (
            $($name:ident : $file:ident),*
        ) => {
            $(pub(crate) mod $file;)*

            #[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub enum Role {
                $($name),*
            }
            impl Role {
                pub fn values() -> Vec<Role> {
                    return vec![$(Role::$name),*];
                }
                pub fn default_state(&self) -> RoleState {
                    match self {
                        $(Self::$name => RoleState::$name($file::$name::default())),*
                    }
                }
                pub fn suspicious(&self) -> bool {
                    match self {
                        $(Self::$name => $file::SUSPICIOUS),*
                    }
                }
                pub fn defense(&self) -> u8 {
                    match self {
                        $(Self::$name => $file::DEFENSE),*
                    }
                }
                pub fn witchable(&self) -> bool {
                    match self {
                        $(Self::$name => $file::WITCHABLE),*
                    }
                }
                pub fn roleblockable(&self) -> bool {
                    match self {
                        $(Self::$name => $file::ROLEBLOCKABLE),*
                    }
                }
                pub fn maximum_count(&self) -> Option<u8> {
                    match self {
                        $(Self::$name => $file::MAXIMUM_COUNT),*
                    }
                }
                pub fn faction_alignment(&self) -> FactionAlignment {
                    match self {
                        $(Self::$name => $file::FACTION_ALIGNMENT),*
                    }
                }
                pub fn end_game_condition(&self) -> EndGameCondition {
                    match self {
                        $(Self::$name => $file::END_GAME_CONDITION),*
                    }
                }
                pub fn team(&self) -> Option<Team> {
                    match self {
                        $(Self::$name => $file::TEAM),*
                    }
                }
            }

            // This does not need to implement Deserialize or PartialEq!
            // Use Role for those things!
            #[derive(Clone, Debug, Serialize)]
            #[serde(tag = "role", rename_all = "camelCase")]
            pub enum RoleState {
                $($name($file::$name)),*
            }
            impl RoleState {
                pub fn role(&self) -> Role {
                    match self {
                        $(Self::$name(_) => Role::$name),*
                    }
                }
                pub fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority){
                    match self {
                        $(Self::$name(role_struct) => role_struct.do_night_action(game, actor_ref, priority)),*
                    }
                }
                pub fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference){
                    match self {
                        $(Self::$name(role_struct) => role_struct.do_day_action(game, actor_ref, target_ref)),*
                    }
                }
                pub fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool{
                    match self {
                        $(Self::$name(role_struct) => role_struct.can_night_target(game, actor_ref, target_ref)),*
                    }
                }
                pub fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool{
                    match self {
                        $(Self::$name(role_struct) => role_struct.can_day_target(game, actor_ref, target_ref)),*
                    }
                }
                pub fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit>{
                    match self {
                        $(Self::$name(role_struct) => role_struct.convert_targets_to_visits(game, actor_ref, target_refs)),*
                    }
                }
                pub fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>{
                    match self {
                        $(Self::$name(role_struct) => role_struct.get_current_send_chat_groups(game, actor_ref)),*
                    }
                }
                pub fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>{
                    match self {
                        $(Self::$name(role_struct) => role_struct.get_current_recieve_chat_groups(game, actor_ref)),*
                    }
                }
                pub fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
                    match self {
                        $(Self::$name(role_struct) => role_struct.on_phase_start(game, actor_ref, phase)),*
                    }
                }
                pub fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
                    match self {
                        $(Self::$name(role_struct) => role_struct.on_role_creation(game, actor_ref)),*
                    }
                }
            }
        }
    }

    macro_rules! priorities {
        (
            $($name:ident),*
        )=>{
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub enum Priority {
                $($name,)*
            }
            impl Priority {
                pub fn values() -> Vec<Self> {
                    return vec![$(Self::$name),*];
                }
            }
        }
    }

    pub(super) use {roles, priorities};
}
