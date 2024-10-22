use serde::{Deserialize, Serialize};

use super::{role_list::RoleOutline, Game};


pub type OutlineIndex = u8;
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default, PartialOrd, Ord)]

pub struct RoleOutlineReference{
    index: OutlineIndex
}

pub type OriginallyGeneratedRoleAndPlayer = (super::role::Role, super::player::PlayerReference);

impl RoleOutlineReference{
    pub fn new(game: &Game, index: OutlineIndex) -> Option<RoleOutlineReference>{
        if (index as usize) >= game.settings.role_list.0.len() { return None;} 
        unsafe { Some(RoleOutlineReference::new_unchecked(index)) }
    }
    pub unsafe fn new_unchecked(index: OutlineIndex) -> RoleOutlineReference {
        RoleOutlineReference { index }
    }
    pub fn index(&self) -> OutlineIndex {
        self.index
    }

    pub fn deref<'a>(&self, game: &'a Game)->&'a RoleOutline{
        &game.settings.role_list.0[self.index as usize]
    }
    pub fn deref_as_role_and_player_originally_generated<'a>(&self, game: &'a Game)->OriginallyGeneratedRoleAndPlayer{
        game.roles_originally_generated[self.index as usize]
    }
}


impl Serialize for RoleOutlineReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_u8(self.index)
    }
}
impl<'a> Deserialize<'a> for RoleOutlineReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        Ok(RoleOutlineReference {
            index: u8::deserialize(deserializer)?
        })
    }
}