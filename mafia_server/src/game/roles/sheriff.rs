use super::*;

create_role! { 
    Sheriff

    "This is the description of the sheriff"

    // night target function

    // day target function

    // etc.

    role-specific data: {}
}

fn a() {
    <Sheriff as Role>::RoleData::new();
}