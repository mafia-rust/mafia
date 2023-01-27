#![allow(unused)]   // TODO remove this eventually

pub mod game;
pub mod lobby;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, MafiaError>;

    pub enum MafiaError {
        Generic(String) // TODO: remove this eventually
    }
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
}