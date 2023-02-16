use crate::game::{phase::Phase, grave::Grave, role::Role, player::{PlayerIndex, Player}, vote::Verdict};
use super::night_message::NightInformationMessage;

#[derive(Clone)]
pub enum MessageSender {
    Player(PlayerIndex),
    Jailor,
    Medium,
}

// Determines message color
#[derive(Clone)]
pub enum ChatMessage {
    Normal(MessageSender, String, ChatGroup),
    Whisper(PlayerIndex, String),
    /* System */
    Debug(String), // TODO: Remove. This is only for testing.
    RoleAssignment(Role),
    PlayerDied(Grave),
    GameOver/*(WinState)*/,
    PhaseChange(Phase),
    /* Trial */
    RequiredVotes(usize),
    Voted { voter: PlayerIndex, votee: PlayerIndex },
    VoteCleared { voter: PlayerIndex },
    PlayerOnTrial(PlayerIndex),
    JudgementVote(PlayerIndex),
    JudgementVerdict(PlayerIndex, Verdict),
    JudgementResults { innocent: usize, guilty: usize },
    NoTrialsLeft,
    /* Misc */
    BroadcastWhisper { whisperer: PlayerIndex, whisperee: PlayerIndex },
    Targeted { targeter: PlayerIndex, target: PlayerIndex },
    TargetCleared { targeter: PlayerIndex },
    /* Role-specific */
    MayorRevealed(PlayerIndex),
}

#[derive(Clone)]
pub enum ChatGroup {
    All,
    Mafia,
    Dead,
    Jail,
    None,
}
