#[macro_use]
mod role;
pub use role::*;

macro_rules! use_roles {
    (
        $($name:ident),*
    ) => {
        $(
            mod $name;
            pub(crate) use $name::*;
        )*
    };
}

use_roles! {
    consigliere,
    consort,
    doctor,
    escort,
    godfather,
    sheriff,
    veteran,
    vigilante
}