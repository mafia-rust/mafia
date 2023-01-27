use crate::game::Game;

pub struct Lobby {

    //the idea behind the lobby thing is that it  allows players to connect before the game started
    //therefore, the game gets created and then starts at the same time
    //we couold have it so a game can be created without being started. You pick if thats a better idea, if it is you can delete this file.
    game: Option<Game>,
}