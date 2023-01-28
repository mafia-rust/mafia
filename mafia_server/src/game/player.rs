use super::roles::Role;
use super::roles::RoleData;

pub type PlayerID = u8;

pub struct Player {
    name : String,
    role_data : Box<dyn RoleData>,
}

impl Player {
    pub fn new<R: Role + 'static>(name : String) -> Self {
        Player {
            name,
            role_data : Box::new(R::RoleData::new()),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}