use serde::Serialize;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum AttackPower {
    Basic = 1,
    ArmorPiercing = 2,
    ProtectionPiercing = 3
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize)]
pub enum DefensePower {
    None = 0,
    Armor = 1,
    Protection = 2,
    Invincible = 3
}

impl AttackPower {
    pub fn can_pierce(self, defense: DefensePower) -> bool {
        self as u8 > defense as u8
    }
    pub fn is_stronger(self, other: AttackPower) -> bool {
        self > other
    }
}
impl DefensePower {
    pub fn can_block(self, attack: AttackPower) -> bool {
        self as u8 >= attack as u8
    }
    pub fn is_stronger(self, other: DefensePower) -> bool {
        self > other
    }
}