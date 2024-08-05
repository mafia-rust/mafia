use serde::{Deserialize, Serialize};

use crate::{game::{player::PlayerReference, Game}, packet::ToClientPacket};

use super::chat::{ChatMessage, ChatMessageVariant};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum PlayerGroup {
    All,
    Dead,

    Mafia,
    Cult,

    Jail,
    Interview
}
impl PlayerGroup{
    pub fn insert_role_label(&self, game: &mut Game, revealed_player: PlayerReference){
        for player in self.players_that_receive_chat(game) {
            if
                revealed_player != player &&
                revealed_player.alive(game) &&
                player.deref_mut(game).role_labels.insert(revealed_player)
            {
                player.add_chat_message(game, ChatMessage::new(
                    ChatMessageVariant::PlayersRoleRevealed { player: revealed_player.index(), role: revealed_player.role(game) }, 
                    Some(*self)
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
                    Some(*self)
                ));
            }
    
            player.send_packet(game, ToClientPacket::YourRoleLabels{
                role_labels: PlayerReference::ref_map_to_index(player.role_label_map(game)) 
            });
        }
    }
    pub fn players_that_receive_chat(&self, game: &Game)->Vec<PlayerReference>{
        let mut out = Vec::new();
        for player_ref in PlayerReference::all_players(game){
            if player_ref.get_current_receive_chat_groups(game).contains(self){
                out.push(player_ref);
            }
        }
        out
    }
}