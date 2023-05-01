#[allow(clippy::module_inception)]
mod player;
mod player_voting_variables;
mod player_night_variables;
mod player_accessors;
pub use player::*;

pub type PlayerIndex = u8;