use super::role::{Role, RoleData};

pub type PlayerID = usize;

pub struct Player {
    name : String,
    role_data : RoleData,
}

impl Player {
    pub fn new(name : String, role: Role) -> Self {
        Player {
            name,
            role_data : role.default_data(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}