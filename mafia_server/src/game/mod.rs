pub mod grave;
pub mod phase;
pub mod player;
pub mod chat;
pub mod role;
pub mod visit;
pub mod verdict;
pub mod role_list;
pub mod settings;
pub mod victory_group;

#[allow(clippy::module_inception)]
mod game;
pub use game::*;