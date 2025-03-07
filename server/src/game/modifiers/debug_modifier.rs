
use crate::game::{chat::{chat_message_variant::ChatMessageVariant, ChatGroup}, player::PlayerReference, role::Priority, Game};

use super::{ModifierTrait, ModifierType};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct DebugModifier;

impl From<&DebugModifier> for ModifierType{
    fn from(_: &DebugModifier) -> Self {
        ModifierType::DebugModifier
    }
}

impl ModifierTrait for DebugModifier {
    fn on_night_priority(self, game: &mut Game, priority: Priority) {
        if priority != Priority::DebugMessages {return;};
        
        let players = PlayerReference::all_players(game);
        for player in players {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::DebugVisits {
                visitor: player,
                visited: player.all_night_visits_cloned(game).iter().map(|visit| visit.target).collect(),
            });
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::Debug {
                text: format!("@{} has role state data: {:?}",player.index(), player.role_state(game))
            });
        }
    }
}