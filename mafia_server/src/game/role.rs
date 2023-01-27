use super::player::PlayerID;

pub enum RoleData {
    Sheriff,
    Lookout,
    Investigator,
    Spy,
    Veteran,
    Vigilante {
        bullets_left: u8, 
        killed_townie: bool
    },
    Doctor,
    Bodyguard,
    Mayor {
        revealed: bool,
    },
    Medium,
    Escort,
    Transporter,
    Godfather,
    Mafioso,
    Consort,
    Blackmailer,
    Consigliere,
    Framer,
    Disguiser,
    Janitor,
    Forger,
    Jester,
    Executioner {
        target: PlayerID,
    },
    Witch,
    Arsonist,
    Werewolf,
    Vampire,
}