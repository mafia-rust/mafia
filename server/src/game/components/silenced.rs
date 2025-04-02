use crate::{game::{chat::ChatMessageVariant, event::on_midnight::MidnightVariables, phase::PhaseType, player::PlayerReference, Game}, packet::ToClientPacket, vec_set::VecSet};

impl Game {
    pub fn silenced(&self)->&Silenced{
        &self.silenced
    }
    pub fn silenced_mut(&mut self) -> &mut Silenced {
        &mut self.silenced
    }
}
// If this comment is still here upon review, then I have not tested this. Please remind me
#[derive(Default, Clone)]
pub struct Silenced {
    silenced_players: VecSet<PlayerReference>,
}
impl Silenced {
    pub fn silence_night(game: &mut Game, midnight_variables: &mut MidnightVariables, player: PlayerReference) {
        game.silenced_mut().silenced_players.insert(player);

        player.push_night_message(midnight_variables, ChatMessageVariant::Silenced);
        player.send_packet(game, ToClientPacket::YourSendChatGroups { send_chat_groups: 
            player.get_current_send_chat_groups(game).into_iter().collect()
        });
    }
    pub fn unsilence(game: &mut Game, player: PlayerReference) {
        game.silenced_mut().silenced_players.remove(&player);
    }
    pub fn silenced(&self, player: PlayerReference) -> bool {
        self.silenced_players.contains(&player)
    }
    pub fn on_phase_start(game: &mut Game, phase: PhaseType) {
        if phase == PhaseType::Night {
            for player in PlayerReference::all_players(game) {
                Silenced::unsilence(game, player);
            }
        }
    }
}