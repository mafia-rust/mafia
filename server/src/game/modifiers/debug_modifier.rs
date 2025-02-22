use crate::game::{chat::chat_message_variant::ChatMessageVariant, player::PlayerReference, role::Priority, Game};

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
            player.push_night_message(game, ChatMessageVariant::DebugVisits { 
                visited_by: player.all_night_visitors_cloned(game), 
                visited: player.all_night_visits_cloned(game) });
        }
        
    }
}