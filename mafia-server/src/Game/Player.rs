use super::RoleData::RoleData;

type PlayerID = String;

pub struct Player{
    name : String,
    role_data : Option<RoleData>,
}