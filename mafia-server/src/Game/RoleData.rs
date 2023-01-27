pub enum RoleData{
    Sheriff,
    Vigilante{bullets_left: u8, killed_townie: bool},
    Jester,
    Mafioso,
}