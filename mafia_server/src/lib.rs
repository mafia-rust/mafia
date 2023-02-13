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