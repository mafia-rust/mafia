macro_rules! make_role_enum {
    (
        $(
            $name:ident : $file:ident $({
                $($data_ident:ident: $data_type:ty = $data_def:expr),*
            })?
        ),*
    ) => {
        use crate::game::player::PlayerIndex;
        use crate::game::visit::Visit;
        use crate::game::Game;
        use crate::game::end_game_condition::EndGameCondition;
        use crate::game::chat::ChatGroup;
        use crate::game::role_list::FactionAlignment;
        use crate::game::phase::PhaseType;
        use serde::{Serialize, Deserialize};
        $(mod $file;)*


        #[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
        pub enum Role {
            $($name),*
        }

        #[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
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
            
            //from file
            pub fn is_suspicious(&self) -> bool {
                match self {
                    $(Role::$name => $file::SUSPICIOUS),*
                }
            }
            pub fn is_witchable(&self) -> bool {
                match self {
                    $(Role::$name => $file::WITCHABLE),*
                }
            }
            pub fn get_defense(&self) -> u8 {
                match self {
                    $(Role::$name => $file::DEFENSE),*
                }
            }
            pub fn is_roleblockable(&self) -> bool {
                match self {
                    $(Role::$name => $file::ROLEBLOCKABLE),*
                }
            }
            pub fn get_faction_alignment(&self) -> FactionAlignment {
                match self {
                    $(Role::$name => $file::FACTION_ALIGNMENT),*
                }
            }
            pub fn get_maximum_count(&self) -> Option<u8> {
                match self {
                    $(Role::$name => $file::MAXIUMUM_COUNT),*
                }
            }
            pub fn get_end_game_condition(&self) -> EndGameCondition {
                match self {
                    $(Role::$name => $file::END_GAME_CONDITION),*
                }
            }
            //Above is constants

            pub fn do_night_action(&self, actor_index: PlayerIndex, priority: i8, game: &mut Game) {
                match self {
                    $(Role::$name => $file::do_night_action(actor_index, priority, game)),*
                }
            }
            pub fn do_day_action(&self, actor_index: PlayerIndex, game: &mut Game) {
                match self {
                    $(Role::$name => $file::do_day_action(actor_index, game)),*
                }
            }
            pub fn can_night_target(&self, actor_index: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
                match self {
                    $(Role::$name => $file::can_night_target(actor_index, target, game)),*
                }
            }
            pub fn can_day_target(&self, actor_index: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
                match self {
                    $(Role::$name => $file::can_day_target(actor_index, target, game)),*
                }
            }
            pub fn convert_targets_to_visits(&self, actor_index: PlayerIndex, targets: Vec<PlayerIndex>, game: &Game) -> Vec<Visit> {
                match self {
                    $(Role::$name => $file::convert_targets_to_visits(actor_index, targets, game)),*
                }
            }
            pub fn get_current_chat_groups(&self, actor_index: PlayerIndex, game: &Game) -> Vec<ChatGroup> {
                match self {
                    $(Role::$name => $file::get_current_chat_groups(actor_index, game)),*
                }
            }
            pub fn on_phase_start(&self, actor_index: PlayerIndex, phase: PhaseType, game: &mut Game){
                match self{
                    $(Role::$name => $file::on_phase_start(actor_index, phase, game)),*
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

// Creates the Role enum
make_role_enum! {
    Sheriff : sheriff,

    Doctor : doctor {
        self_heals_remaining: u8 = 1,
        target_healed_index: Option<PlayerIndex> = None
    },

    Veteran : veteran {
        alerts_remaining: u8 = 3,
        alerting_tonight: bool = false
    },

    Mafioso : mafioso {
        original: bool = true
    },
    
    Consort : consort
}

type Priority = i8;

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