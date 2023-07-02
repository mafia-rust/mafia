mod player_accessors;
mod player_reference;
mod player_send_packet;
mod player_reset;
mod player_helper_functions;

pub use player_reference::PlayerIndex;
pub use player_reference::PlayerReference;

use std::collections::HashMap;
use std::time::Duration;

use crate::{
    game::{
        role::{Role, RoleState}, 
        chat::ChatMessage, 
        visit::Visit, 
        grave::{GraveRole, GraveKiller}, 
        verdict::Verdict, available_buttons::AvailableButtons
    },
    websocket_connections::connection::ClientSender,
};

use super::recruit::Recruit;
use super::tag::Tag;

pub struct Player {
    connection: ClientConnection,

    name: String,
    role_state: RoleState,
    alive: bool,
    will: String,
    notes: String,
    death_note: Option<String>,

    role_labels: HashMap<PlayerReference, Role>,   //when you can see someone elses role in the playerlist, dead players and teammates, mayor
    player_tags: HashMap<PlayerReference, Vec<Tag>>,


    pub chat_messages: Vec<ChatMessage>, //all messages
    queued_chat_messages: Vec<ChatMessage>, //messages that have yet to have been sent to the client

    last_sent_buttons: Vec<AvailableButtons>,

    voting_variables: PlayerVotingVariables,
    night_variables: PlayerNightVariables,

    #[allow(dead_code)]
    recruit: Option<Recruit>,
}
struct PlayerVotingVariables{
    chosen_vote:    Option<PlayerReference>,
    verdict:        Verdict,
}
struct PlayerNightVariables{
    died:           bool,
    attacked:       bool,
    jailed:         bool,
    roleblocked:    bool,
    defense:        u8,

    appeared_visits: Option<Vec<Visit>>,
    appeared_role:    Role,

    silenced:       bool,

    chosen_targets: Vec<PlayerReference>,
    visits:         Vec<Visit>,

    messages: Vec<ChatMessage>,

    grave_role: GraveRole,
    grave_killers: Vec<GraveKiller>,
    grave_will: String,
    grave_death_notes: Vec<String>,
}
impl Player {
    pub fn new(name: String, sender: ClientSender, role: Role) -> Self {
        Self {
            connection: ClientConnection::Connected(sender),

            name,
            role_state: role.default_state(),
            alive: true,
            will: "".to_string(),
            notes: "".to_string(),
            death_note: None,

            role_labels: HashMap::new(),
            player_tags: HashMap::new(),


            chat_messages: Vec::new(),
            queued_chat_messages: Vec::new(),
            
            last_sent_buttons: Vec::new(),

            voting_variables: PlayerVotingVariables{
                chosen_vote : None,
                verdict : Verdict::Abstain,
            },
            night_variables: PlayerNightVariables{
                died:               false,
                attacked:           false,
                jailed:             false,
                roleblocked:        false,
                defense:            0,
                appeared_visits:    None,
                appeared_role:      role,

                silenced:           false,

                chosen_targets:     vec![],
                visits:             vec![],

                messages:           vec![],

                grave_role: GraveRole::Role(role),
                grave_killers: vec![],
                grave_will: "".to_string(),
                grave_death_notes: vec![],
            },

            recruit: None
        }
    }
}

pub mod test {
    use std::{collections::HashMap, time::Duration};

    use crate::game::{role::Role, verdict::Verdict, grave::GraveRole};

    use super::{Player, PlayerVotingVariables, PlayerNightVariables};

    pub fn mock_player(name: String, role: Role) -> Player {
        Player {
            // Since `tick` is never called in tests, this will never decrement.
            connection: super::ClientConnection::CouldReconnect { disconnect_timer: Duration::from_secs(1) },

            name,
            role_state: role.default_state(),
            alive: true,
            will: "".to_string(),
            notes: "".to_string(),
            death_note: None,

            role_labels: HashMap::new(),
            player_tags: HashMap::new(),


            chat_messages: Vec::new(),
            queued_chat_messages: Vec::new(),
            
            last_sent_buttons: Vec::new(),

            voting_variables: PlayerVotingVariables{
                chosen_vote : None,
                verdict : Verdict::Abstain,
            },
            night_variables: PlayerNightVariables{
                died:               false,
                attacked:           false,
                jailed:             false,
                roleblocked:        false,
                defense:            0,
                appeared_visits:    None,
                appeared_role:      role,

                silenced:           false,

                chosen_targets:     vec![],
                visits:             vec![],

                messages:           vec![],

                grave_role: GraveRole::Role(role),
                grave_killers: vec![],
                grave_will: "".to_string(),
                grave_death_notes: vec![],
            },

            recruit: None
        }
    }
}

pub const DISCONNECT_TIMER_SECS: u64 = 45;

enum ClientConnection {
    Connected(ClientSender),
    CouldReconnect { disconnect_timer: Duration },
    Disconnected
}
