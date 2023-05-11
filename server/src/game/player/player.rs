use std::collections::HashMap;

use tokio::sync::mpsc::UnboundedSender;

use crate::{game::{role::{RoleData, Role}, chat::ChatMessage, visit::Visit, grave::{GraveRole, GraveKiller}, verdict::Verdict}, network::packet::ToClientPacket};

use super::{PlayerIndex, PlayerReference};


pub struct Player {
    pub(super) sender: UnboundedSender<ToClientPacket>,


    pub(super) name: String,
    pub(super) index: PlayerIndex,
    pub(super) role_data: RoleData,
    pub(super) alive: bool,
    pub(super) will: String,
    pub(super) notes: String,

    pub(super) role_labels: HashMap<PlayerReference, Role>,   //when you can see someone elses role in the playerlist, dead players and teammates, mayor


    pub(super) chat_messages: Vec<ChatMessage>,
    pub(super) queued_chat_messages: Vec<ChatMessage>, 

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
    pub(super) suspicious:     bool,

    pub(super) disguised_as:   Option<PlayerReference>,

    pub(super) chosen_targets: Vec<PlayerReference>,
    pub(super) visits:         Vec<Visit>,

    pub(super) messages: Vec<ChatMessage>,

    pub(super) grave_role: GraveRole,
    pub(super) grave_killers: Vec<GraveKiller>,
    pub(super) grave_will: String,
    pub(super) grave_death_notes: Vec<String>,
}
impl Player {
    pub fn new(index: PlayerIndex, name: String, sender: UnboundedSender<ToClientPacket>, role: Role) -> Self {
        Self {
            name,
            index,
            role_data: role.default_data(),
            alive: true,
            will: "".to_string(),
            notes: "".to_string(),

            role_labels: HashMap::new(),

            sender,
            chat_messages: Vec::new(),

            queued_chat_messages: Vec::new(),

            night_variables: PlayerNightVariables{
                alive_tonight:  true,
                died:           false,
                attacked:       false,
                jailed:         false,
                roleblocked:    false,
                defense:        0,
                suspicious:     false,

                disguised_as:   None,

                chosen_targets: vec![],
                visits:         vec![],

                messages: vec![],

                grave_role: GraveRole::Role(Role::Sheriff), //This should not be a problem because we reset immedietly on creation
                grave_killers: vec![],
                grave_will: "".to_string(),
                grave_death_notes: vec![]
            },
            voting_variables: PlayerVotingVariables{
                chosen_vote : None,
                verdict : Verdict::Abstain,
            }
        }
    }
}
