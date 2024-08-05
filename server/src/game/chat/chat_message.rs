use serde::{Deserialize, Serialize};

use crate::{game::{player::PlayerReference, player_group::PlayerGroup, Game}, packet::ToClientPacket};

use super::chat_message_variant::ChatMessageVariant;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage{
    pub variant: ChatMessageVariant,
    pub recipient: Recipient,
}

impl ChatMessage{
    pub fn new(variant: ChatMessageVariant, recipient: impl Into<Recipient>)->Self{
        Self{variant, recipient: recipient.into()}
    }
    pub fn send(self, game: &mut Game) {
        game.add_message(self.recipient, self.variant)
    }
    pub fn get_variant(&self)->&ChatMessageVariant{
        &self.variant
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum Recipient {
    Player(PlayerReference),
    Group(PlayerGroup)
}

impl From<PlayerGroup> for Recipient {
    fn from(value: PlayerGroup) -> Self {
        Recipient::Group(value)
    }
}

impl From<PlayerReference> for Recipient {
    fn from(value: PlayerReference) -> Self {
        Recipient::Player(value)
    }
}

impl Recipient {
    pub fn insert_role_label(&self, game: &mut Game, revealed_player: PlayerReference){
        for player in self.players_that_receive_chat(game) {
            if
                revealed_player != player &&
                revealed_player.alive(game) &&
                player.deref_mut(game).role_labels.insert(revealed_player)
            {
                player.add_chat_message(game, ChatMessage::new(
                    ChatMessageVariant::PlayersRoleRevealed { player: revealed_player.index(), role: revealed_player.role(game) }, 
                    *self
                ));
            }
    
            player.send_packet(game, ToClientPacket::YourRoleLabels{
                role_labels: PlayerReference::ref_map_to_index(player.role_label_map(game)) 
            });
        }
    }
    pub fn remove_role_label(&self, game: &mut Game, concealed_player: PlayerReference){
        for player in self.players_that_receive_chat(game) {
            if player.deref_mut(game).role_labels.remove(&concealed_player) {
                player.add_chat_message(game, ChatMessage::new(
                    ChatMessageVariant::PlayersRoleConcealed { player: concealed_player.index() },
                    *self
                ));
            }
    
            player.send_packet(game, ToClientPacket::YourRoleLabels{
                role_labels: PlayerReference::ref_map_to_index(player.role_label_map(game)) 
            });
        }
    }
    pub fn contains(&self, game: &Game, player_ref: PlayerReference) -> bool {
        match self {
            Self::Player(p) => player_ref == *p,
            Self::Group(group) => player_ref.get_current_receive_chat_groups(game).contains(group)
        }
    }
    pub fn players_that_receive_chat(&self, game: &Game)->Vec<PlayerReference>{
        if let Self::Player(player_ref) = self {
            vec![*player_ref]
        } else {
            PlayerReference::all_players(game)
                .filter(|player_ref| self.contains(game, *player_ref))
                .collect()
        }
    }
}