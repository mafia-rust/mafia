use std::time::Duration;

use crate::{packet::ToClientPacket, websocket_connections::connection::ClientSender};

#[derive(Clone, Debug)]
pub enum ClientConnection {
    Connected(ClientSender),
    CouldReconnect { disconnect_timer: Duration },
    Disconnected
}
impl ClientConnection {
    pub fn send_packet(&self, packet: ToClientPacket)->bool {
        if let ClientConnection::Connected(ref sender) = self {
            sender.send(packet);
            true
        }else{
            false
        }
    }
}