

use crate::game::player::PlayerReference;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::chat::ChatGroup;
use crate::game::role_list::FactionAlignment;
use crate::game::phase::PhaseType;
use crate::game::team::Team;

use serde::{Serialize, Deserialize};


macro_rules! make_role_enum {
    (
        $(
            $name:ident : $file:ident
        ),*
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
                    $(
                        Self::$name(_role_struct) => Role::$name
                    ),*
                }
            }
        }

        impl PlayerReference{
            pub fn role_state_do_night_action(self, game: &mut Game, priority: Priority){
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().do_night_action(game, self, priority)
                    ),*
                }
            }
            pub fn role_state_do_day_action(self, game: &mut Game, target_ref: PlayerReference){
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().do_day_action(game, self, target_ref)
                    ),*
                }
            }
            pub fn role_state_can_night_target(self, game: &Game, target_ref: PlayerReference) -> bool{
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().can_night_target(game, self, target_ref)
                    ),*
                }
            }
            pub fn role_state_can_day_target(self, game: &Game, target_ref: PlayerReference) -> bool{
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().can_night_target(game, self, target_ref)
                    ),*
                }
            }
            pub fn role_state_convert_targets_to_visits(self, game: &Game, target_refs: Vec<PlayerReference>) -> Vec<Visit>{
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().convert_targets_to_visits(game, self, target_refs)
                    ),*
                }
            }
            pub fn role_state_get_current_send_chat_groups(self, game: &Game) -> Vec<ChatGroup>{
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().get_current_send_chat_groups(game, self)
                    ),*
                }
            }
            pub fn role_state_get_current_recieve_chat_groups(self, game: &Game) -> Vec<ChatGroup>{
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().get_current_recieve_chat_groups(game, self)
                    ),*
                }
            }
            pub fn role_state_on_phase_start(self, game: &mut Game, phase: PhaseType){
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().on_phase_start(game, self, phase)
                    ),*
                }
            }
            pub fn role_state_on_role_creation(self, game: &mut Game){
                let role_state = self.role_state(game).clone();
                match role_state {
                    $(
                        Self::$name(role_struct) => role_struct.clone().on_role_creation(game, self)
                    ),*
                }
            }
        }
    }
}

pub trait RoleStateImpl {
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
make_role_enum! {
    Jailor : jailor,

    // Sheriff : sheriff,
    // Lookout : lookout,

    // Doctor : doctor,
    // Bodyguard : bodyguard,

    Vigilante : vigilante
    // Veteran : veteran,

    // Escort : escort,
    // Medium : medium,
    // Retributionist : retributionist,

    // Mafioso : mafioso,
    
    // Consort : consort,
    // Blackmailer : blackmailer,

    // Janitor : janitor,
    // Framer : framer,

    // CovenLeader : coven_leader,

    // VoodooMaster : voodoo_master,

    // Jester : jester
}

macro_rules! make_priority_enum {
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
make_priority_enum! {
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


/*
New Proposed priorities:

Top
    Jester(Kill), Vigilante(Suicide)
Unswappable
    Arsonist(Clear self), All decidedes (Vet decide)
    Ritualist, Doomsayer?
Transporter
    Transporter(Swap)
Controlls
    Witch(Swap), 
Necromany,
    Retributionist(Swap) Necromancer(Swap)
Roleblock
    Escort Consort Poisoner(roleblock)
Deception
    Arsonist(Douse), Werewolf(Make slef inno or sus)
    Blackmailer, Janitor(Clean), Forger(Yea)
    HexMaster(Hex), Enchanter(Alter/Enchant), Poisoner(Poison), Illusionist, Dreamweaver(Choose to dreamweave), VoodooMaster, Medusa
    Shroud(make harmful)
Trickster
    Trickster(Swap)
Bodyguard
    Bodyguard(Swap)
Heal
    Doctor, PotionMaster(Heal), Veteran(Heal self) Bodyguard(Heal self), PotionMaser(protect), Trapper(Build/Place/Protect), Crusader(Heal)
Kill
    Ambusher CovenLeader, Necronomicon, Arsonist(Ignite) HexMaster(Kill) Veteran(Kill) Poisoner(Kill) PotionMaser(kill) Trapper(kill)
    Jinx, Shroud(kill), Crusader(Kill) Jailor(Kill)
Investigative
    Sheriff, Investigator, Lookout, Tracker, Trapper(Investigate)
    Spy(Mafia/Coven visits + bug), Seer, Psychic, Coroner, Wildling
    Janitor(Who died) Bodyguard(Notif) Doctor(Notif) Arsonist(Who visited me) PotionMaser(reveal)
StealMessages
    Witch(steal messages)
    Retributionist(steal messages)
    Necromancer(steal messages)
    


*/
