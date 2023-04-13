#![allow(unused)]   // TODO remove this eventually

pub mod game;
pub mod lobby;
pub mod network;

#[macro_use]
pub mod prelude {
    pub type Result<T> = std::result::Result<T, MafiaError>;

    #[derive(Debug)]
    pub enum MafiaError {
        Generic(String) // TODO: remove this eventually
    }

    macro_rules! err {
        (generic, $($arg:tt)*) => {
            MafiaError::Generic(format!($($arg)*))
        }
    }

    pub(crate) use err;
}

// TODO: remove if we never use this
pub mod macros {
    macro_rules! enum_str {
        (enum $name:ident {
            $($variant:ident = $val:expr),*,
        }) => {
            enum $name {
                $($variant = $val),*
            }
    
            impl $name {
                fn name(&self) -> &'static str {
                    match self {
                        $($name::$variant => stringify!($variant)),*
                    }
                }
            }
        };
    }
    
    pub(crate) use enum_str;
}
pub mod utils{
    use std::fmt::Display;

    /**
    Converts x to any radix
    # Panics
    radix < 2 || radix > 36
    # Example
    ```
    assert_eq!(format_radix(7, 2), "111");
    assert_eq!(format_radix(366, 10), "366");
    assert_eq!(format_radix(36*36*36*36 - 1, 36), "zzzz");
    ```
    */
    #[allow(unused)]
    fn format_radix(mut x: u32, radix: u32) -> Option<String> {
        let mut result = vec![];

        loop {
            let m = x % radix;
            x = x / radix;
            
            result.push(std::char::from_digit(m, radix)?);
            if x == 0 {
                break;
            }
        }
        Some(result.into_iter().rev().collect())
    }

    pub fn trim_whitespace(s: &str) -> String {
        let mut new_str = s.trim().to_owned();
        let mut prev = ' '; // The initial value doesn't really matter
        new_str.retain(|ch| {
            let result = ch != ' ' || prev != ' ';
            prev = ch;
            result
        });
        new_str
    }

}

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