use super::*;

create_role! { 
    Veteran

    "This is the description of the veteran"

    // night target function

    // day target function

    // etc.

    role-specific data: {
        alerts_remaining: u8 = 1
    }
}

struct AdditionalVeteranData {
    alerts_remaining: u8
}