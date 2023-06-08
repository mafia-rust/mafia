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


pub struct Player {
    pub(super) sender: Option<ClientSender>,

    pub(super) name: String,
    pub(super) role_state: RoleState,
    pub(super) alive: bool,
    pub(super) will: String,
    pub(super) notes: String,

    pub(super) role_labels: HashMap<PlayerReference, Role>,   //when you can see someone elses role in the playerlist, dead players and teammates, mayor


    pub(super) chat_messages: Vec<ChatMessage>, //all messages
    pub(super) queued_chat_messages: Vec<ChatMessage>, //messages that have yet to have been sent to the client

    pub(super) last_sent_buttons: Vec<AvailableButtons>,

    pub(super) voting_variables: PlayerVotingVariables,
    pub(super) night_variables: PlayerNightVariables,
}
pub(super) struct PlayerVotingVariables{
    pub(super) chosen_vote:    Option<PlayerReference>,
    pub(super) verdict:        Verdict,
}
pub(super) struct PlayerNightVariables{
    pub(super) alive_tonight:  bool,
    pub(super) died:           bool,
    pub(super) attacked:       bool,
    pub(super) jailed:         bool,
    pub(super) roleblocked:    bool,
    pub(super) defense:        u8,

    pub(super) appeard_suspicious:     bool,
    pub(super) appeared_visits: Vec<Visit>,

    pub(super) silenced:       bool,

    pub(super) chosen_targets: Vec<PlayerReference>,
    pub(super) visits:         Vec<Visit>,

    pub(super) messages: Vec<NightInformation>,

    pub(super) grave_role: GraveRole,
    pub(super) grave_killers: Vec<GraveKiller>,
    pub(super) grave_will: String,
    pub(super) grave_death_notes: Vec<String>,
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


            chat_messages: Vec::new(),
            queued_chat_messages: Vec::new(),
            
            last_sent_buttons: Vec::new(),

            voting_variables: PlayerVotingVariables{
                chosen_vote : None,
                verdict : Verdict::Abstain,
            },
            night_variables: PlayerNightVariables{
                alive_tonight:  true,
                died:           false,
                attacked:       false,
                jailed:         false,
                roleblocked:    false,
                defense:        0,
                appeard_suspicious:false,
                appeared_visits: vec![],

                silenced:       false,

                chosen_targets: vec![],
                visits:         vec![],

                messages: vec![],

                grave_role: GraveRole::Role(Role::Sheriff), //This should not be a problem because we reset immedietly on creation
                grave_killers: vec![],
                grave_will: "".to_string(),
                grave_death_notes: vec![],
            },
        }
    }
}