use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::game::Game;

use super::Player;

pub type PlayerIndex = u8;
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct PlayerReference {
    index: PlayerIndex
}

#[derive(Debug)]
pub struct InvalidPlayerReferenceError;

impl PlayerReference{
    pub fn new(game: &Game, index: PlayerIndex) -> Result<PlayerReference, InvalidPlayerReferenceError>{
        if (index as usize) >= game.players.len() { return Err(InvalidPlayerReferenceError);} 
        Ok(PlayerReference::new_unchecked(index))
    }
    pub fn new_unchecked(index: PlayerIndex) -> PlayerReference {
        PlayerReference { index }
    }
    pub fn deref<'a>(&self, game: &'a Game)->&'a Player{
        &game.players[self.index as usize]
    }
    pub fn deref_mut<'a>(&self, game: &'a mut Game)->&'a mut Player{
        &mut game.players[self.index as usize]
    }
    pub fn index(&self) -> PlayerIndex {
        self.index
    }

    pub fn ref_option_to_index(option: &Option<PlayerReference>) -> Option<PlayerIndex>{
        option.as_ref().map(PlayerReference::index)
    }
    pub fn ref_vec_to_index(ref_vec: &[PlayerReference]) -> Vec<PlayerIndex>{
        ref_vec.iter().map(PlayerReference::index).collect()
    }
    pub fn ref_map_to_index<T>(ref_map: HashMap<PlayerReference, T>) -> HashMap<PlayerIndex, T> {
        ref_map.into_iter().map(|(player_ref, value)| {
            (player_ref.index(), value)
        }).collect()
    }
    
    pub fn index_option_to_ref(game: &Game, index_option: &Option<PlayerIndex>)->Result<Option<PlayerReference>, InvalidPlayerReferenceError>{
        index_option
            .map(|index| PlayerReference::new(game, index))
            .transpose()
    }
    pub fn index_vec_to_ref(game: &Game, index_vec: &Vec<PlayerIndex>)->Result<Vec<PlayerReference>, InvalidPlayerReferenceError>{
        let mut out = Vec::new();
        for index in index_vec{
            out.push(match Self::new(game, *index){
                Ok(player_ref) => player_ref,
                Err(e) => return Err(e),
            });
        }
        Ok(out)
    }

    pub fn all_players(game: &Game)->Vec<PlayerReference>{
        let mut out = Vec::new();
        for player_index in 0..game.players.len(){
            // Guaranteed to be safe
            out.push(PlayerReference::new_unchecked(player_index as PlayerIndex));
        }
        out
    }
}


impl Serialize for PlayerReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_u8(self.index)
    }
}
impl<'a> Deserialize<'a> for PlayerReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'a> {
        Ok(PlayerReference {
            index: u8::deserialize(deserializer)?
        })
    }
}