mod player_accessors;
mod player_reference;
mod player_send_packet;
mod player_reset;
mod player_helper_functions;

pub use player_reference::PlayerIndex;
pub use player_reference::PlayerReference;

use std::collections::HashMap;

use crate::{
    game::{
        role::{Role, RoleState}, 
        chat::{
            ChatMessage, 
            night_message::NightInformation
        }, 
        visit::Visit, 
        grave::{GraveRole, GraveKiller}, 
        verdict::Verdict, available_buttons::AvailableButtons
    },
    websocket_connections::connection::ClientSender,
};

use super::tag::Tag;


pub struct Player {
    sender: Option<ClientSender>,

    name: String,
    role_state: RoleState,
    alive: bool,
    will: String,
    notes: String,

    role_labels: HashMap<PlayerReference, Role>,   //when you can see someone elses role in the playerlist, dead players and teammates, mayor
    player_tags: HashMap<PlayerReference, Vec<Tag>>,


    chat_messages: Vec<ChatMessage>, //all messages
    queued_chat_messages: Vec<ChatMessage>, //messages that have yet to have been sent to the client

    last_sent_buttons: Vec<AvailableButtons>,

    voting_variables: PlayerVotingVariables,
    night_variables: PlayerNightVariables,
}
struct PlayerVotingVariables{
    chosen_vote:    Option<PlayerReference>,
    verdict:        Verdict,
}
struct PlayerNightVariables{
    alive_tonight:  bool,
    died:           bool,
    attacked:       bool,
    jailed:         bool,
    roleblocked:    bool,
    defense:        u8,

    appeared_visits: Vec<Visit>,
    appeared_role:    Role,

    silenced:       bool,

    chosen_targets: Vec<PlayerReference>,
    visits:         Vec<Visit>,

    messages: Vec<NightInformation>,

    grave_role: GraveRole,
    grave_killers: Vec<GraveKiller>,
    grave_will: String,
    grave_death_notes: Vec<String>,
}
impl Player {
    pub fn new(name: String, sender: ClientSender, role: Role) -> Self {
        Self {
            sender: Some(sender),

            name,
            role_state: role.default_state(),
            alive: true,
            will: "".to_string(),
            notes: "".to_string(),

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
                alive_tonight:      true,
                died:               false,
                attacked:           false,
                jailed:             false,
                roleblocked:        false,
                defense:            0,
                appeared_visits:    vec![],
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