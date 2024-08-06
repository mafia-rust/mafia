use serde::{Deserialize, Serialize};
use vec1::Vec1;

use crate::{game::{player::PlayerReference, player_group::PlayerGroup, tag::Tag, Game}, packet::ToClientPacket};

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

impl Recipient {
    pub fn add_chat_message(&self, game: &mut Game, message: ChatMessageVariant) {
        let message = ChatMessage::new(message, *self);

        for player in self.players(game) {
            player.deref_mut(game).chat_messages.push(message.clone());
            player.deref_mut(game).queued_chat_messages.push(message.clone());
        }
    }

    pub fn send_chat_message(&self, game: &mut Game, message: ChatMessageVariant) {
        self.add_chat_message(game, message.clone());
        for player in self.players(game) {
            player.send_chat_messages(game);
        }

        if let Recipient::Group(PlayerGroup::All) = self {
            game.send_chat_message_to_spectators(message)
        }
    }

    pub fn send_chat_messages(&mut self, game: &mut Game, variants: Vec<ChatMessageVariant>) {
        for message in variants.into_iter(){
            self.send_chat_message(game, message);
        }
    }

    pub fn insert_role_label(&self, game: &mut Game, revealed_player: PlayerReference) {
        for player in self.players(game) {
            if
                revealed_player != player &&
                revealed_player.alive(game) &&
                player.deref_mut(game).role_labels.insert(revealed_player)
            {
                player.add_chat_message(game, ChatMessageVariant::PlayersRoleRevealed { player: revealed_player.index(), role: revealed_player.role(game) });
            }
    
            player.send_packet(game, ToClientPacket::YourRoleLabels{
                role_labels: PlayerReference::ref_map_to_index(player.role_label_map(game)) 
            });
        }
    }

    pub fn remove_role_label(&self, game: &mut Game, concealed_player: PlayerReference) {
        for player in self.players(game) {
            if player.deref_mut(game).role_labels.remove(&concealed_player) {
                player.add_chat_message(game, ChatMessageVariant::PlayersRoleConcealed { player: concealed_player.index() });
            }
    
            player.send_packet(game, ToClientPacket::YourRoleLabels{
                role_labels: PlayerReference::ref_map_to_index(player.role_label_map(game)) 
            });
        }
    }

    pub fn push_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag) {
        for player in self.players(game) {
            if let Some(player_tags) = player.deref_mut(game).player_tags.get_mut(&key){
                player_tags.push(value);
            }else{
                player.deref_mut(game).player_tags.insert(key, vec1::vec1![value]);
            }
            player.add_chat_message(game, ChatMessageVariant::TagAdded { player: key.index(), tag: value });
            player.send_packet(game, ToClientPacket::YourPlayerTags { 
                player_tags: PlayerReference::ref_map_to_index(player.deref(game).player_tags.clone())
            });
        }
    }

    pub fn remove_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag) {
        for player in self.players(game) {
            let Some(player_tags) = player.deref_mut(game).player_tags.get_mut(&key) else {continue};

            match Vec1::try_from_vec(
                player_tags.clone()
                    .into_iter()
                    .filter(|t|*t!=value)
                    .collect()
            ){
                Ok(new_player_tags) => {
                    *player_tags = new_player_tags
                },
                Err(_) => {
                    player.deref_mut(game).player_tags.remove(&key);
                },
            }
            player.add_chat_message(game, ChatMessageVariant::TagRemoved { player: key.index(), tag: value });
            player.send_packet(game, ToClientPacket::YourPlayerTags{
                player_tags: PlayerReference::ref_map_to_index(player.deref(game).player_tags.clone())
            });
        }
    }

    pub fn remove_player_tag_on_all(&self, game: &mut Game, value: Tag) {
        for player_ref in PlayerReference::all_players(game){
            self.remove_player_tag(game, player_ref, value)
        }
    }

    pub fn contains(&self, game: &Game, player_ref: PlayerReference) -> bool {
        match self {
            Self::Player(p) => player_ref == *p,
            Self::Group(group) => player_ref.get_current_receive_chat_groups(game).contains(group)
        }
    }

    pub fn players(&self, game: &Game)->Vec<PlayerReference> {
        if let Self::Player(player_ref) = self {
            vec![*player_ref]
        } else {
            PlayerReference::all_players(game)
                .filter(|player_ref| self.contains(game, *player_ref))
                .collect()
        }
    }
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

pub trait RecipientLike {
    fn add_chat_message(&self, game: &mut Game, message: ChatMessageVariant);
    fn send_chat_message(&self, game: &mut Game, message: ChatMessageVariant);
    fn send_chat_messages(&mut self, game: &mut Game, variants: Vec<ChatMessageVariant>);
    fn insert_role_label(&self, game: &mut Game, revealed_player: PlayerReference);
    fn remove_role_label(&self, game: &mut Game, concealed_player: PlayerReference);
    fn push_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag);
    fn remove_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag);
    fn remove_player_tag_on_all(&self, game: &mut Game, value: Tag);
}

impl<R> RecipientLike for R
where Recipient: From<R>, R: Copy
{
    fn add_chat_message(&self, game: &mut Game, message: ChatMessageVariant) {
        Recipient::from(*self).add_chat_message(game, message)
    }

    fn send_chat_message(&self, game: &mut Game, message: ChatMessageVariant) {
        Recipient::from(*self).send_chat_message(game, message)
    }

    fn send_chat_messages(&mut self, game: &mut Game, variants: Vec<ChatMessageVariant>) {
        Recipient::from(*self).send_chat_messages(game, variants)
    }

    fn insert_role_label(&self, game: &mut Game, revealed_player: PlayerReference) {
        Recipient::from(*self).insert_role_label(game, revealed_player);
    }

    fn remove_role_label(&self, game: &mut Game, concealed_player: PlayerReference) {
        Recipient::from(*self).remove_role_label(game, concealed_player);
    }

    fn push_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag) {
        Recipient::from(*self).push_player_tag(game, key, value);
    }

    fn remove_player_tag(&self, game: &mut Game, key: PlayerReference, value: Tag) {
        Recipient::from(*self).remove_player_tag(game, key, value);
    }

    fn remove_player_tag_on_all(&self, game: &mut Game, value: Tag) {
        Recipient::from(*self).remove_player_tag_on_all(game, value);
    }
}