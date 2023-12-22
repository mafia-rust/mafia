mod player_accessors;
mod player_reference;
mod player_send_packet;
mod player_reset;
mod player_helper_functions;

pub use player_reference::PlayerIndex;
pub use player_reference::PlayerReference;

use std::collections::HashMap;

use crate::lobby::ClientConnection;
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

use super::tag::Tag;

pub struct Player {
    connection: ClientConnection,

    name: String,
    role_state: RoleState,
    alive: bool,
    will: String,
    notes: String,
    death_note: Option<String>,

    role_labels: HashMap<PlayerReference, Role>,
    player_tags: HashMap<PlayerReference, Vec<Tag>>,


    pub chat_messages: Vec<ChatMessage>,
    queued_chat_messages: Vec<ChatMessage>, // Not yet sent to the client

    last_sent_buttons: Vec<AvailableButtons>,

    voting_variables: PlayerVotingVariables,
    night_variables: PlayerNightVariables,
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
        }
    }
}

pub mod test {
    use std::{collections::HashMap, time::Duration};

    use crate::{game::{role::Role, verdict::Verdict, grave::GraveRole}, lobby::ClientConnection};

    use super::{Player, PlayerVotingVariables, PlayerNightVariables};

    pub fn mock_player(name: String, role: Role) -> Player {
        Player {
            // Since `tick` is never called in tests, this will never decrement.
            connection: ClientConnection::CouldReconnect { disconnect_timer: Duration::from_secs(1) },

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
        }
    }
}
