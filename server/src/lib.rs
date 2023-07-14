pub mod game;
pub mod websocket_connections;
pub mod listener;
pub mod lobby;
pub mod packet;

pub mod log {
    #[macro_export]
    /// Log a statement to the console. 
    /// When logging using this macro, a timestamp and possibly a marker is added to the message.
    macro_rules! log {
        // Each case in this macro definition is for a different log marker.
        // None
        ($expr:expr) => {
            println!("\x1b[0;90m{}\x1b[0m {}", chrono::Local::now().format("%m.%d %I:%M:%S"), $expr)
        };
        // Fatal error
        (fatal $prefix:expr; $($expr:expr),*) => {
            log!(&format!("\x1b[0;1;91m[{}] FATAL\x1b[0m \x1b[0;1;41m{}\x1b[0m", $prefix, &format!($($expr),*)))
        };
        // Warning error
        (error $prefix:expr; $($expr:expr),*) => {
            log!(&format!("\x1b[0;1;91m[{}] WARN\x1b[0m {}", $prefix, &format!($($expr),*)))
        };
        // Important
        (important $prefix:expr; $($expr:expr),*) => {
            log!(&format!("\x1b[0;1;93m[{}]\x1b[0m {}", $prefix, &format!($($expr),*)))
        };
        // Info
        (info $prefix:expr; $($expr:expr),*) => {
            log!(&format!("\x1b[0;1;32m[{}]\x1b[0m {}", $prefix, &format!($($expr),*)))
        };
        // Default (use info)
        ($prefix:expr; $($expr:expr),*) => {
            log!(info $prefix; $($expr),*)
        };
    }
}
pub mod strings{
    pub trait TidyableString {
        fn trim_whitespace(&self) -> Self;
        fn remove_newline(&self) -> Self;
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
        fn remove_newline(&self) -> String {
            let mut new_str = self.trim().to_owned();
            new_str.retain(|ch| {ch != '\n'});
            new_str
        }
        /// Removes more than two newlines in a row
        fn trim_newline(&self) -> String {
            let mut new_str = self.trim().to_owned();
            let mut prev = (' ', ' '); // The initial value doesn't really matter
            new_str.retain(|ch| {
                //if theyre not all newlines, keep the character
                let result = ch != '\n' || prev.0 != '\n' || prev.1 != '\n';
                (prev.0, prev.1) = (prev.1, ch);
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
