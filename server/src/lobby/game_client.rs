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