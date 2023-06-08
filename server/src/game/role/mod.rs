

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
        // $(use crate::game::role::$file::$name;)*

        #[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum Role {
            $($name),*
        }
        impl Role {
            pub fn values()->Vec<Role>{
                return vec![$(Role::$name),*];
            }
            pub fn get_default_role_functions(&self)->RoleState{
                match self {
                    $(Self::$name => $file::$name::default()),*
                }
            }
        }

        #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
        #[serde(tag = "role", rename_all = "camelCase")]
        pub enum RoleState {
            $($name($file::$name)),*
        }
        impl RoleState{
            pub fn get_role_functions(&self)->impl RoleFunctions{
                match self {
                    $(Self::$name(role_struct) => role_struct),*
                }
            }
            pub fn role(&self) -> Role {
                match self {
                    $(
                        Self::$name(role_struct) => Role::$name
                    ),*
                }
            }
        }
    }
}

pub trait RoleFunctions {
    fn suspicious() -> bool;
    fn defense() -> u8;
    fn witchable() -> bool;
    fn roleblockable() -> bool;
    fn maximum_count() -> Option<u8>;
    fn faction_alignment(&self) -> FactionAlignment;
    fn end_game_condition() -> EndGameCondition;
    fn team() -> Option<Team>;

    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority);
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference);
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool;
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool;
    fn convert_targets_to_visits(self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit>;
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>;
    fn get_current_recieve_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup>;
    fn on_phase_start(self,  game: &mut Game, actor_ref: PlayerReference, phase: PhaseType);
    fn on_role_creation(self,  game: &mut Game, actor_ref: PlayerReference);
}

// Creates the Role enum
make_role_enum! {
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

    Mafioso : mafioso,
    
    Consort : consort,
    Blackmailer : blackmailer,

    Janitor : janitor,
    Framer : framer,

    CovenLeader : coven_leader,

    VoodooMaster : voodoo_master,

    Jester : jester
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