use super::role::RoleData;

pub type PlayerID = u8;

pub struct Player {
    name : String,
    role_data : Option<RoleData>,
}