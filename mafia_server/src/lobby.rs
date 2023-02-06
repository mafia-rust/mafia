use crate::{game::{Game, player::PlayerID}, network::connection::Connection};

pub struct Lobby {
    game: Option<Game>,
}

type LobbyID = String;

impl Lobby {
    pub fn new() -> Lobby {
        Self { 
            game: None,
        }
    }
}