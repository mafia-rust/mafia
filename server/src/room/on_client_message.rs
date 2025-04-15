use std::time::{Duration, Instant};

use crate::{game::on_client_message::GameAction, lobby::on_client_message::LobbyAction, log, packet::{ToClientPacket, ToServerPacket}, websocket_connections::connection::ClientSender};

use super::{RoomClientID, Room};

pub const MESSAGE_PER_SECOND_LIMIT: u16 = 1;
pub const MESSAGE_PER_SECOND_LIMIT_TIME: Duration = Duration::from_secs(10);

pub enum RoomAction {
    LobbyAction(LobbyAction),
    GameAction(GameAction),
    None,
} 

impl Room {
    pub fn on_client_message(&mut self, send: &ClientSender, room_client_id: RoomClientID, incoming_packet: ToServerPacket) -> RoomAction {
        //RATE LIMITER
        match incoming_packet {
            ToServerPacket::Judgement { .. } |
            ToServerPacket::SendChatMessage { .. } |
            ToServerPacket::SendLobbyMessage { .. } |
            ToServerPacket::SendWhisper { .. } => {
                let Some(last_message_times) = (match self {
                    Self::Game(game) => game.get_client_last_message_times(room_client_id),
                    Self::Lobby(lobby) => lobby.get_client_last_message_times(room_client_id)
                }) else {
                    log!(error "Room"; "{} {:?}", "Message recieved from player not in game", incoming_packet);
                    return RoomAction::None;
                };

                let now = Instant::now();
                while let Some(time) = last_message_times.front() {
                    if now.duration_since(*time) > MESSAGE_PER_SECOND_LIMIT_TIME {
                        last_message_times.pop_front();
                    } else {
                        break;
                    }
                }
                if last_message_times.len() as u64 >= MESSAGE_PER_SECOND_LIMIT_TIME.as_secs().saturating_mul(MESSAGE_PER_SECOND_LIMIT.into()) {
                    send.send(ToClientPacket::RateLimitExceeded);
                    return RoomAction::None;
                }
                last_message_times.push_back(now);
                
            },
            _ => {}
        }

        match self {
            Self::Game(game) => RoomAction::GameAction(game.on_client_message(send, room_client_id, incoming_packet)),
            Self::Lobby(lobby) => RoomAction::LobbyAction(lobby.on_client_message(send, room_client_id, incoming_packet))
        }
    }
}