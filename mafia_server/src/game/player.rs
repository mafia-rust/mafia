use super::role::Role;

pub type PlayerID = usize;

pub struct Player {
    name : String,
    role_data : Role,
}

impl Player {
    pub fn new(name : String, role: Role) -> Self {
        Player {
            name,
            role_data : role,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}