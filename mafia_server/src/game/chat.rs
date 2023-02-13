use super::{phase::Phase, grave::Grave, role::Role, player::PlayerIndex};


pub enum MessageSender {
    Player(PlayerIndex),
    Jailor,
    Medium,
}

//determines message color
pub enum ChatMessageType {
    Normal(MessageSender, String),

    Whisper(PlayerIndex, String),    //jack whispered "amongus"
    WhisperPublic(PlayerIndex, PlayerIndex), //Jack whispering to sammy

    Targeted(PlayerIndex, PlayerIndex), //Jack targeted willow
    Voted(PlayerIndex, PlayerIndex),    //sammy voted jack
    JudgementVote(PlayerIndex), //Sammy has voted
    JudgementVerdict(PlayerIndex, i32),    //Sammy voted guilty

    PlayerOnTrial,

}
pub struct  ChatMessage {
    message: ChatMessageType,
}

// Maybe change this to an enum? E.g. ChatGroup::All, ChatGroup::Mafia, ChatGroup::Dead, etc.
pub struct ChatGroup {
    pub players: Vec<PlayerIndex>
}