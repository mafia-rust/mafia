pub mod spectator_pointer;

use crate::{
    client_connection::ClientConnection,
    packet::ToClientPacket,
    game::chat::ChatMessageVariant,
};

#[derive(Debug, Clone)]
pub struct SpectatorInitializeParameters {
    pub connection: ClientConnection,
    pub host: bool,
}
pub struct Spectator {
    pub connection: ClientConnection,
    pub fast_forward_vote: bool,

    pub queued_chat_messages: Vec<ChatMessageVariant>,
}
impl Spectator {
    pub fn new(params: SpectatorInitializeParameters) -> Self {
        Self {
            connection: params.connection,
            fast_forward_vote: false,

            queued_chat_messages: Vec::new(),
        }
    }
    pub fn send_packet(&self, packet: ToClientPacket) {
        self.connection.send_packet(packet);
    }
    pub fn send_packets(&self, packets: Vec<ToClientPacket>) {
        for packet in packets {
            self.send_packet(packet);
        }
    }
}