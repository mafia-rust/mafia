use mafia_server::{game::{player::{PlayerReference, PlayerIndex}, Game, chat::ChatMessage}, packet::ToServerPacket};

#[derive(Clone, Copy, Debug)]
pub struct TestPlayer(PlayerReference, *mut Game);

/// A macro to get the game from this TestPlayer.
/// ## Example:
/// ```
/// // In TestPlayer::can_day_target
/// assert!(self.0.can_day_target(game!(self), target.0));

/// game!(self).on_client_message(self.0.index(), 
///     ToServerPacket::DayTarget { player_index: target.index() }
/// );
/// ```
macro_rules! game {
    ($self:ident) => {unsafe {&mut *$self.1}}
}

impl TestPlayer {
    pub fn new(player: PlayerReference, game: &Game) -> Self {
        TestPlayer(player, game as *const Game as *mut Game)
    }

    pub fn index(&self) -> PlayerIndex {
        self.0.index()
    }

    pub fn set_night_targets(&self, targets: Vec<TestPlayer>) {
        for target in targets.clone() {
            assert!(self.0.can_night_target(game!(self), target.0));
        }
        self.0.set_chosen_targets(game!(self), targets.into_iter().map(|t|t.0).collect());
    }

    pub fn set_night_target(&self, target: TestPlayer) {
        assert!(self.0.can_night_target(game!(self), target.0));
        self.0.set_chosen_targets(game!(self), vec![target.0]);
    }

    pub fn send_message(&self, message: &str) {
        game!(self).on_client_message(self.0.index(), 
            ToServerPacket::SendMessage { text: message.to_string() }
        );
    }

    pub fn day_target(&self, target: TestPlayer) {
        assert!(self.0.can_day_target(game!(self), target.0));

        game!(self).on_client_message(self.0.index(), 
            ToServerPacket::DayTarget { player_index: target.index() }
        );
    }

    pub fn alive(&self) -> bool {
        self.0.alive(game!(self))
    }

    pub fn die(&self) {
        assert!(self.alive());
        self.0.set_alive(game!(self), false)
    }

    pub fn was_roleblocked(&self) -> bool {
        self.0.night_roleblocked(game!(self))
    }

    pub fn get_messages(&self) -> &Vec<ChatMessage> {
        &self.0.deref(game!(self)).chat_messages
    }
}