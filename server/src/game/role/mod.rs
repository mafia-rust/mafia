macro_rules! make_role_enum {
    (
        $(
            $name:ident : $file:ident $({
                $($data_ident:ident: $data_type:ty = $data_def:expr),*
            })?
        ),*
    ) => {
        $(pub(crate) mod $file;)*


        #[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub enum Role {
            $($name),*
        }

        #[derive(Clone, PartialEq, Debug)]
        pub enum RoleData {
            $($name $({
                $($data_ident: $data_type),*
            })?),*
        }

        impl Role {  
            pub fn values() -> Vec<Role> {
                return vec![$(Role::$name),*];
            }
            pub fn default_data(&self) -> RoleData {
                match self {
                    $(Role::$name => RoleData::$name$({
                        $($data_ident: $data_def),*
                    })?),*
                }
            }
            
            pub fn suspicious(&self) -> bool {
                match self {
                    $(Role::$name => $file::SUSPICIOUS),*
                }
            }
            pub fn witchable(&self) -> bool {
                match self {
                    $(Role::$name => $file::WITCHABLE),*
                }
            }
            pub fn defense(&self) -> u8 {
                match self {
                    $(Role::$name => $file::DEFENSE),*
                }
            }
            pub fn roleblockable(&self) -> bool {
                match self {
                    $(Role::$name => $file::ROLEBLOCKABLE),*
                }
            }
            pub fn faction_alignment(&self) -> FactionAlignment {
                match self {
                    $(Role::$name => $file::FACTION_ALIGNMENT),*
                }
            }
            pub fn maximum_count(&self) -> Option<u8> {
                match self {
                    $(Role::$name => $file::MAXIUMUM_COUNT),*
                }
            }
            pub fn end_game_condition(&self) -> EndGameCondition {
                match self {
                    $(Role::$name => $file::END_GAME_CONDITION),*
                }
            }
            pub fn team(&self) -> Option<Team> {
                match self {
                    $(Role::$name => $file::TEAM),*
                }
            }

            pub fn do_night_action(&self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
                match self {
                    $(Role::$name => $file::do_night_action(game, actor_ref, priority)),*
                }
            }
            pub fn do_day_action(&self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
                match self {
                    $(Role::$name => $file::do_day_action(game, actor_ref, target_ref)),*
                }
            }
            pub fn can_night_target(&self,  game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
                match self {
                    $(Role::$name => $file::can_night_target(game, actor_ref, target_ref)),*
                }
            }
            pub fn can_day_target(&self,  game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
                match self {
                    $(Role::$name => $file::can_day_target(game, actor_ref, target_ref)),*
                }
            }
            pub fn convert_targets_to_visits(&self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
                match self {
                    $(Role::$name => $file::convert_targets_to_visits(game, actor_ref, target_refs)),*
                }
            }
            pub fn get_current_send_chat_groups(&self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
                match self {
                    $(Role::$name => $file::get_current_send_chat_groups(game, actor_ref)),*
                }
            }
            pub fn get_current_recieve_chat_groups(&self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
                match self {
                    $(Role::$name => $file::get_current_recieve_chat_groups(game, actor_ref)),*
                }
            }
            pub fn on_phase_start(&self,  game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
                match self{
                    $(Role::$name => $file::on_phase_start(game, actor_ref, phase)),*
                }
            }
            pub fn on_role_creation(&self,  game: &mut Game, actor_ref: PlayerReference){
                match self{
                    $(Role::$name => $file::on_role_creation(game, actor_ref)),*
                }
            }
        }

        impl RoleData {
            pub fn role(&self) -> Role {
                match self {
                    $(RoleData::$name$({
                        $($data_ident: _),*
                    })? => Role::$name),*
                }
            }
        }
    }
}


use crate::game::player::{PlayerIndex, PlayerReference};
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::chat::ChatGroup;
use crate::game::role_list::FactionAlignment;
use crate::game::phase::PhaseType;
use crate::game::team::Team;
use serde::{Serialize, Deserialize};
// Creates the Role enum
make_role_enum! {
    Jailor : jailor {
        jailed_target_ref: Option<PlayerReference> = None
    },

    Sheriff : sheriff,

    Doctor : doctor {
        self_heals_remaining: u8 = 1,
        target_healed_ref: Option<PlayerReference> = None
    },

    Veteran : veteran {
        alerts_remaining: u8 = 3,
        alerting_tonight: bool = false
    },

    Escort : escort,

    Mafioso : mafioso,
    
    Consort : consort,

    Janitor : janitor {
        cleans_remaining: u8 = 3,
        cleaned_ref: Option<PlayerReference> = None
    },

    CovenLeader : coven_leader,

    VoodooMaster : voodoo_master
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

1 Top
    Jailor, Jester(Kill), Vigilante(Suicide)
1 Unswappable
    Arsonist(Clear self), All decidedes (Vet decide)
    Ritualist, Doomsayer?

3 Swaps
    Transporter(Swap)
5 Controlls
    Witch(Swap), Retributionist(Swap) Necromancer(Swap)
6 Roleblock
    Escort Consort Poisoner(roleblock)


7 Deception
    Arsonist(Douse), Janitor(Clean), Forger(Yea), Disguiser, Werewolf(Make slef inno or sus)
    HexMaster(Hex), Enchanter(Alter/Enchant), Poisoner(Poison), Illusionist, Dreamweaver(Choose to dreamweave), VoodooMaster, Medusa
    Shroud(make harmful)
8 Deception Effected Swaps
    Trickster(Swap)
9
    Bodyguard(Swap)
10 Heal
    Doctor, PotionMaster(Heal), Veteran(Heal self) Bodyguard(Heal self), PotionMaser(protect), Trapper(Build/Place/Protect), Crusader(Heal)

11 Kill
    Ambusher CovenLeader, Necronomicon, Arsonist(Ignite) HexMaster(Kill) Veteran(Kill) Poisoner(Kill) PotionMaser(kill) Trapper(kill)
    Jinx, Shroud(kill), Crusader(Kill) Jailor(Kill)
12 Investigative
    Sheriff, Investigator, Lookout, Tracker, Trapper(Investigate)
    Spy(Mafia/Coven visits + bug), Seer, Psychic, Coroner, Wildling
    Janitor(Who died) Bodyguard(Notif) Doctor(Notif) Arsonist(Who visited me) PotionMaser(reveal)
13
    Witch(steal messages)
    


 */
/*
Proposed Priorities:

Visit objects created key:
nv = no visit
av = astral visit
v = visit

Highest importance
+1: Jester(Kill, av) Vigilante(Suicide, nv) Arsonist(Clear self, nv) Vampire(Choose Leader, nv) Witch(Activate sheild, nv) Veteran(Decide, av) Retributionist(Decide and witch, av, av) //non transportable or witchable
Non roleblockable Swaps
+2: Transporter(Swaps, v, v)
+3: Witch(Swap, v, av) 
RB
+4: Escort/Consort(Roleblock, v)
Swaps
+5: Godfather("witch mafioso if theyre not rbd, clear targets on self", av)
+6: Doctor(Heal, v) Bodyguard("Witch attacker", v) //all attacks should happen after this
Deception
+7: Blackmailer, Arsonist(Douse&visitors, v&nv), Framer(Frame&visitors, v&nv), Disguiser("Swap", v, v) Werewolf("unframe", nv) Forger(Frame, v) Janitor(Clean, v)   //investigations happen after this
Investigation
+8: Invest, Sheriff, Lookout, Tracker, Consig
Kills
+9: Mafioso/Godfather/Sk/Ww/Vet/Vig/Vamp/Arso/Bg/Vig("Kill & make grave")
Notify
+10: Doc(Notify both, nv) Bg(Notify both, nv) Janitor(notify, nv), Forger(notify, nv) Vamp(Inform Leader & new vamp, nv) Arsonist(Inform who is doused, nv)
Convert + bug
+11: spy(bug, v)
+12: Witch(bug)

//graves made

After night is over
    Exe convert
    mafioso convert
    Amne(Convert)
    Vamp(Convert)
*/






/*
Old Priorities:

-12: Veteran(Decides Alert) Vigilante(Suicide) Jester(Kill) Arsonist(Clean self) Vampire(Choose Leader)
-10: Transporter(Swaps)
-8: Witch(Swaps, Activate sheild)
-7: Retributionist(Choose to revive)
-6: Escort / Consort(Roleblock)
-4: Godfather(Swap mafioso target and clear self)
-2 bodyguard(swap)
0: visits happen here
+2: Doctor(Heal), Blackmailer(Decide), Crusader(Heal), Arsonist(Douse), Framer, Disguiser Werewolf(innos themself)
+4: Sheriff, Invest, Consig, Lookout, Tracker, Arsonist(Find who visited)
+6: Mafioso/Godfather, SerialKiller, Werewolf, Veteran, Vampire, Arsonist, Crusader, Bodyguard, Vigilante (All kill)
+8: Forger(Change info), Janitor(Clean & info), Doctor(Notify) Bodyguard(Notify) 
+10: Spy(Collect info) Vampire(Inform of leader) Arsonist(Inform who is doused)
+11: Amnesiac(Convert) Vampire(Convert) Executioner(convert)
+12: Witch(Steal info & Remove sheild)
*/