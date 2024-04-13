#![allow(clippy::single_match)]
#![allow(clippy::get_first)]

use crate::game::player::PlayerReference;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::chat::ChatGroup;
use crate::game::phase::PhaseType;

use serde::{Serialize, Deserialize};

use super::end_game_condition::EndGameCondition;

trait RoleStateImpl: Clone + std::fmt::Debug + Serialize + Default {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8;

    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority);
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference);

    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool;
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool;

    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit>;

    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>;
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>;

    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool;

    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType);
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference);
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference);
    fn on_game_ending(self, game: &mut Game, actor_ref: PlayerReference);
}

// Creates the Role enum
macros::roles! {
    Jailor : jailor,

    Sheriff : sheriff,
    Lookout : lookout,
    Spy : spy,
    Tracker : tracker,
    Seer : seer,
    Psychic : psychic,
    Auditor : auditor,

    Doctor : doctor,
    Bodyguard : bodyguard,
    Crusader : crusader,
    Reveler : reveler,
    Trapper : trapper,

    Vigilante : vigilante,
    Veteran : veteran,
    Deputy : deputy,

    Escort : escort,
    Medium : medium,
    Retributionist : retributionist,
    Journalist : journalist,
    Mayor : mayor,
    Transporter : transporter,

    // Mafia
    Godfather : godfather,
    Mafioso : mafioso,
    
    Consort : consort,
    Blackmailer : blackmailer,
    Consigliere: consigliere,
    Witch : witch,
    Necromancer : necromancer,

    Janitor : janitor,
    Framer : framer,
    Forger : forger,

    // Neutral
    Jester : jester,
    Executioner : executioner,
    Politician : politician,

    Doomsayer : doomsayer,
    Death : death,

    Arsonist : arsonist,
    Werewolf : werewolf,
    Ojo : ojo,

    Amnesiac : amnesiac,
    Martyr : martyr,

    Apostle : apostle,
    Disciple : disciple,
    Zealot : zealot
}

macros::priorities! {
    TopPriority,

    Transporter,

    Control,
    Roleblock,

    Deception,

    Bodyguard,

    Heal,
    Kill,
    Investigative,

    SpyBug,

    StealMessages,

    Convert,

    SpyCultistCount
}

mod common_role;

mod macros {
    macro_rules! roles {
        (
            $($name:ident : $file:ident),*
        ) => {
            $(pub mod $file;)*

            #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, PartialOrd, Ord)]
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
                pub fn maximum_count(&self) -> Option<u8> {
                    match self {
                        $(Self::$name => $file::MAXIMUM_COUNT),*
                    }
                }
                pub fn faction(&self) -> crate::game::role_list::Faction {
                    match self {
                        $(Self::$name => $file::FACTION),*
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
                pub fn defense(&self, game: &Game, actor_ref: PlayerReference) -> u8 {
                    match self {
                        $(Self::$name(role_struct) => role_struct.defense(game, actor_ref)),*
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
                pub fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>{
                    match self {
                        $(Self::$name(role_struct) => role_struct.get_current_receive_chat_groups(game, actor_ref)),*
                    }
                }
                pub fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool{
                    match self {
                        $(Self::$name(role_struct) => role_struct.get_won_game(game, actor_ref)),*
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
                pub fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
                    match self {
                        $(Self::$name(role_struct) => role_struct.on_any_death(game, actor_ref, dead_player_ref)),*
                    }
                }
                pub fn on_game_ending(self, game: &mut Game, actor_ref: PlayerReference){
                    match self {
                        $(Self::$name(role_struct) => role_struct.on_game_ending(game, actor_ref)),*
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
#[allow(clippy::match_like_matches_macro)]
impl Role{
    pub fn control_immune(&self)->bool{
        match self {
            Role::Veteran => true,

            Role::Transporter => true,
            
            Role::Retributionist => true,
            Role::Witch => true,
            Role::Necromancer => true,
            
            Role::Doomsayer => true,

            Role::Ojo => true,
            _ => false,
        }
    }
    pub fn roleblock_immune(&self)->bool{
        match self {
            Role::Veteran => true,
            
            Role::Transporter => true,
            Role::Reveler => true,

            Role::Escort => true,
            Role::Consort => true,
            
            Role::Retributionist => true,
            Role::Witch => true,
            Role::Necromancer => true,
            _ => false,
        }
    }
    pub fn has_innocent_aura(&self, game: &Game)->bool{
        match self {
            Role::Jester => true,
            Role::Executioner => true,
            Role::Godfather => true,
            Role::Werewolf => {
                game.day_number() == 1 || game.day_number() == 3
            },
            _ => false,
        }
    }
    pub fn has_suspicious_aura(&self, _game: &Game)->bool{
        match self {
            Role::Politician => true,
            _ => false,
        }
    }
    pub fn end_game_condition(&self)->EndGameCondition{
        EndGameCondition::from_role(*self)
    }
}
pub fn same_evil_team(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    (actor_ref.role(game).faction() == super::role_list::Faction::Mafia && target_ref.role(game).faction() == super::role_list::Faction::Mafia) ||
    (actor_ref.role(game).faction() == super::role_list::Faction::Cult && target_ref.role(game).faction() == super::role_list::Faction::Cult)
}