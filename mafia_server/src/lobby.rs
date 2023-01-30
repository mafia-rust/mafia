use crate::{game::{Game, player::PlayerID}, connection::Connection};

pub struct Lobby {
    game: Option<Game>,
}

impl Lobby{
    pub fn new()->Lobby{

        let new = Self { 
            game: None,
        };

        new
    }
}