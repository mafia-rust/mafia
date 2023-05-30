#![allow(unused)]   // TODO remove this eventually

pub mod game;
pub mod websocket_connections;
pub mod listener;
pub mod lobby;
pub mod packet;

pub mod log {
    pub fn important(str: &str) -> String {
        format!("\x1b[0;1;93m{str}\x1b[0m")
    }
    pub fn error(str: &str) -> String {
        format!("\x1b[0;1;91m{str}\x1b[0m")
    }
    pub fn notice(str: &str) -> String {
        format!("\x1b[0;32m{str}\x1b[0m")
    }
}