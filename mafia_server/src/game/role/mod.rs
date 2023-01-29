#[macro_use]
mod macros;

// Creates the Role enum
make_role_enum! {
    Consigliere : consigliere,
    Consort : consort,
    Doctor : doctor,
    Escort : escort,
    Godfather : godfather,
    Sheriff : sheriff,
    Veteran : veteran {
        alerts_remaining: u8
    },
    Vigilante : vigilante {
        bullets_remaining: u8,
        killed_townie: bool
    }
}