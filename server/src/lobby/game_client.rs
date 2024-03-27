use std::{collections::VecDeque, time::Instant};

use crate::game::player::PlayerIndex;

#[derive(Clone, Debug)]
pub struct GameClient{
    pub client_location: GameClientLocation,
    pub host: bool,

    pub last_message_times: VecDeque<Instant>,
}
#[derive(Clone, Debug)]
pub enum GameClientLocation {
    Player(PlayerIndex),
    Spectator
}
impl GameClient {
    pub fn new_spectator(host: bool)->Self{
        GameClient{
            client_location: GameClientLocation::Spectator,
            host,
            last_message_times: VecDeque::new(),
        }
    }
    pub fn set_host(&mut self) {
        self.host = true;
    }
}