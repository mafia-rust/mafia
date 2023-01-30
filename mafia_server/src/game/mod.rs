pub(crate) mod grave;
pub(crate) mod phase;
pub(crate) mod player;

// Make paths to game less redundant
// Instead of crate::game::game::Game, we can just use crate::game::Game
mod game;
pub use game::*;