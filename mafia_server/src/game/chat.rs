use super::{phase::Phase, grave::Grave, role::Role, player::PlayerIndex};

pub enum ChatMessage {
    System {
        title: Option<String>,
        msg: Option<String>,
    },
    Player {
        name: String,
        msg: String,
    },
}

// Maybe change this to an enum? E.g. ChatGroup::All, ChatGroup::Mafia, ChatGroup::Dead, etc.
pub struct ChatGroup {
    pub players: Vec<PlayerIndex>
}