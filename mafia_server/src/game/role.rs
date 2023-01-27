use super::player::PlayerID;

pub enum RoleData {
    Sheriff,
    Veteran{
        alerts_remaining: u8
    },
    Vigilante{
        bullets_remaining: u8, 
        killed_townie: bool
    },

    Doctor, 

    Mayor{revealed: bool},
    Escort,

    Godfather,

    Consort, Consigliere,
}