use super::player::PlayerReference;
use super::Game;



#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AttackType{
    None,
    //true if a possessed dead player cannot attack if this is active
    Attack{possess_immune: bool},
    //if you can attack when your dead
    AttackDead,
    NecroPossess{town_only: bool},
    Revive{town_only: bool},
    Wildcard,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackData {
    pub attack_type: AttackType,
    //Only needs to be correct if the player can attack
    //and reviving the player could help
    pub town_on_grave: bool,
}

impl AttackData {
    pub fn wildcard() -> AttackData {
        AttackData {
            attack_type: AttackType::Wildcard,
            town_on_grave: false,
        }
    }
    pub fn none() -> AttackData {
        AttackData {
            attack_type: AttackType::None,
            town_on_grave: false,
        }
    }
    pub fn attack(game: &Game, actor_ref: PlayerReference, possess_immune: bool) -> AttackData {
        AttackData {
            attack_type: AttackType::Attack{possess_immune},
            town_on_grave: actor_ref.town_on_grave(game),
        }
    }
    pub fn necro(game: &Game, actor_ref: PlayerReference, town_only: bool) -> AttackData {
        AttackData {
            attack_type: AttackType::NecroPossess{town_only},
            town_on_grave: actor_ref.town_on_grave(game)
        }
    }
    pub fn revive(town_only: bool) -> AttackData {
        AttackData {
            attack_type: AttackType::NecroPossess{town_only},
            town_on_grave: false
        }
    }
    pub fn attack_dead() -> AttackData {
        AttackData { 
            attack_type: AttackType::AttackDead, 
            town_on_grave: false
        }
    }
    pub fn is_attack(&self) -> bool {
        matches!(&self.attack_type, AttackType::Attack{..})
    }
    pub fn is_revive(&self) -> bool {
        matches!(&self.attack_type, AttackType::Revive {..})
    }
    pub fn is_attack_dead(&self) -> bool {
        matches!(&self.attack_type, AttackType::AttackDead)
    }
    pub fn is_none(&self) -> bool {
        matches!(&self.attack_type, AttackType::None)
    }
    pub fn is_wildcard(&self) -> bool {
        matches!(&self.attack_type, AttackType::Wildcard)
    }

    pub fn can_revive_to_attack(&self, other: &Self) -> bool {
        match (&self.attack_type, &other.attack_type) {
            (AttackType::Revive{town_only}, AttackType::Attack{..}) => {
                !*town_only || other.town_on_grave
            }
            _=> false,
        }
    }

    pub fn can_possess_to_attack(&self, other: &Self) -> bool {
        match (&self.attack_type, &other.attack_type) {
            (AttackType::NecroPossess{town_only}, AttackType::Attack{possess_immune}) => {
                *possess_immune && (!*town_only || other.town_on_grave)
            }
            _=> false,
        }
    }
}