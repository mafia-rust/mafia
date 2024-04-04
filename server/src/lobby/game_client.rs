use std::{collections::VecDeque, time::Instant};

use crate::game::{player::PlayerIndex, spectator::spectator_pointer::SpectatorIndex};

#[derive(Clone, Debug)]
pub struct GameClient{
    pub client_location: GameClientLocation,
    pub host: bool,

    pub last_message_times: VecDeque<Instant>,
}
#[derive(Clone, Debug)]
pub enum GameClientLocation {
    Player(PlayerIndex),
    Spectator(SpectatorIndex)
}
impl GameClient {
    pub fn new_spectator(index: SpectatorIndex, host: bool)->Self{
        GameClient{
            client_location: GameClientLocation::Spectator(index),
            host,
            last_message_times: VecDeque::new(),
        }
    }
    pub fn set_host(&mut self) {
        self.host = true;
    }
}