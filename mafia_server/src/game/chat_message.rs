use super::{phase::Phase, grave::Grave, role::Role, player::PlayerID};

pub enum ChatMessage {
    Generic {   // Blackmailed, witched, revealed mayor, etc.
        title: Option<String>,
        msg: Option<String>,
    },
    PlayerMessage {
        player_id: PlayerID,
        msg: String,
    },
    PhaseChange(Phase),
    Grave(Grave),
    GameOver /*{
        blablabla win state
    }*/,
    RoleAssignment(Role),
}