
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AttackPower {
    Basic = 1,
    ArmorPiercing = 2,
    ProtectionPiercing = 3
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
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
    pub fn is_stronger(self, other: Self) -> bool {
        self as u8 > other as u8
    }
    pub fn max(a: Self, b: Self)->Self{
        if a.is_stronger(b) {
            a
        }else{
            b
        }
    }
}
impl DefensePower {
    pub fn can_block(self, attack: AttackPower) -> bool {
        self as u8 >= attack as u8
    }
    pub fn is_stronger(self, other: Self) -> bool {
        self as u8 > other as u8
    }
    pub fn max(a: Self, b: Self)->Self{
        if a.is_stronger(b) {
            a
        }else{
            b
        }
    }
}