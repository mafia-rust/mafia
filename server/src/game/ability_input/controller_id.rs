use serde::{Deserialize, Serialize};

use crate::game::{player::PlayerReference, role::Role, Game};

use super::{
    AbilitySelection, BooleanSelection,
    SavedController, StringSelection,
};

pub type RoleControllerID = u8;
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag="type")]
pub enum ControllerID{
    #[serde(rename_all = "camelCase")]
    Role{
        player: PlayerReference,
        role: Role,
        id: RoleControllerID
    },
    ForfeitVote{player: PlayerReference},
    PitchforkVote{player: PlayerReference},
    Nominate{player: PlayerReference},

    ForwardMessage{player: PlayerReference},


    SyndicateGunItemShoot,
    SyndicateGunItemGive,
    SyndicateChooseBackup,
    SyndicateBackupAttack,

    WardenLiveOrDie{
        warden: PlayerReference,
        player: PlayerReference,
    }
}
impl ControllerID{
    pub fn role(player: PlayerReference, role: Role, id: RoleControllerID)->Self{
        Self::Role{player, role, id}
    }
    pub fn nominate(player: PlayerReference)->Self{
        Self::Nominate{player}
    }
    pub fn forfeit_vote(player: PlayerReference)->Self{
        Self::ForfeitVote{player}
    }
    pub fn pitchfork_vote(player: PlayerReference)->Self{
        Self::PitchforkVote{player}
    }
    pub fn syndicate_gun_item_shoot()->Self{
        Self::SyndicateGunItemShoot
    }
    pub fn syndicate_gun_item_give()->Self{
        Self::SyndicateGunItemGive
    }
    pub fn syndicate_choose_backup()->Self{
        Self::SyndicateChooseBackup
    }
    pub fn syndicate_backup_attack()->Self{
        Self::SyndicateBackupAttack
    }
}


impl ControllerID{
    pub fn should_send_chat_message(&self)->bool{
        !matches!(self, ControllerID::Nominate { .. } | ControllerID::ForwardMessage { .. })
    }
    fn get_controller<'a>(&self, game: &'a Game)->Option<&'a SavedController>{
        game.saved_controllers.saved_controllers.get(self)
    }
    pub fn get_selection<'a>(&self, game: &'a Game)->Option<&'a AbilitySelection>{
        let saved_controller = self.get_controller(game)?;
        Some(saved_controller.selection())
    }


    pub fn get_boolean_selection<'a>(&self, game: &'a Game)->Option<&'a BooleanSelection>{
        self.get_selection(game)
            .and_then(|selection| 
                if let AbilitySelection::Boolean(selection) = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }
    pub fn get_string_selection<'a>(&self, game: &'a Game)->Option<&'a StringSelection>{
        self.get_selection(game)
            .and_then(|selection| 
                if let AbilitySelection::String(selection) = selection {
                    Some(selection)
                }else{
                    None
                }
            )
    }
}