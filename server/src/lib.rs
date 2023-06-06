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
    pub trait TidyableString {
        fn trim_whitespace(&self) -> Self;
        fn trim_newline(&self) -> Self;
        fn truncate(&self, max_chars: usize) -> Self;
        fn truncate_lines(&self, max_lines: usize) -> Self;
    }
    impl TidyableString for String {
        /// Removes multiple whitespace in a row.
        fn trim_whitespace(&self) -> String {
            let mut new_str = self.trim().to_owned();
            let mut prev = ' '; // The initial value doesn't really matter
            new_str.retain(|ch| {
                //if theyre not both spaces, keep the character
                let result = ch != ' ' || prev != ' ';
                prev = ch;
                result
            });
            new_str
        }
        /// Removes newlines
        fn trim_newline(&self) -> String {
            let mut new_str = self.trim().to_owned();
            let mut prev = ' '; // The initial value doesn't really matter
            new_str.retain(|ch| {
                //if theyre not both spaces, keep the character
                let result = ch != '\n' || prev != '\n';
                prev = ch;
                result
            });
            new_str
        }
        /// Truncates to a given number of unicode characters, rather than rust's [`String::truncate`]
        /// which truncates to a given number of unicode codepoints.
        fn truncate(&self, max_chars: usize) -> String {
            match self.char_indices().nth(max_chars) {
                None => self.clone(),
                Some((idx, _)) => self[..idx].to_string(),
            }
        }
        /// Truncates to a given number of lines
        fn truncate_lines(&self, max_lines: usize) -> String {
            let out: String = self.lines()
                .take(max_lines)
                .map(|str| str.to_string() + "\n")
                .collect();

            // Remove trailing newline
            out[0..(out.len().saturating_sub(1))].to_string()
        }
    }
}
