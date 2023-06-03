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
pub mod strings{
    ///removes multiple whitespace in a row.
    pub fn trim_whitespace(s: &str) -> String {
        let mut new_str = s.trim().to_owned();
        let mut prev = ' '; // The initial value doesn't really matter
        new_str.retain(|ch| {
            //if theyre not both spaces, keep the character
            let result = ch != ' ' || prev != ' ';
            prev = ch;
            result
        });
        new_str
    }
    ///removes multiple whitespace in a row.
    pub fn trim_new_line(s: &str) -> String {
        let mut new_str = s.trim().to_owned();
        let mut prev = ' '; // The initial value doesn't really matter
        new_str.retain(|ch| {
            //if theyre not both spaces, keep the character
            let result = ch != '\n' || prev != '\n';
            prev = ch;
            result
        });
        new_str
    }
}
