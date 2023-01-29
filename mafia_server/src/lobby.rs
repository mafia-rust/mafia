use crate::game::{Game, player::PlayerID};

pub struct Lobby {
    game: Option<Game>,
}

impl Lobby{
    pub async fn new()->Lobby{

        let new = Self { 
            game: None,
        };
        new
    }
}