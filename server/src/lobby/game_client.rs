use std::{collections::VecDeque, time::Instant};

use serde::Serialize;

use crate::game::{player::PlayerIndex, spectator::spectator_pointer::SpectatorIndex};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameClient{
    #[serde(rename = "clientType")]
    pub client_location: GameClientLocation,
    pub host: bool,

    #[serde(skip)]
    pub last_message_times: VecDeque<Instant>,
}
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "index", rename_all="camelCase")]
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
    pub fn relinquish_host(&mut self) {
        self.host = false;
    }
}