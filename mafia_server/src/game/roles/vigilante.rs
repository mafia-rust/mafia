use super::*;

create_role! { 
    Vigilante

    "This is the description of the vigilante"

    // night target function

    // day target function

    // etc.

    ROLE_SPECIFIC_DATA: {
        bullets_remaining: u8 = 1,
        killed_townie: bool = false
    }
}