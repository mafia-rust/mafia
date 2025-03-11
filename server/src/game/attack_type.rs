use super::player::PlayerReference;
use super::Game;



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttackType{
    None,
    Attack{possess_immune: bool, transport_immune: bool},
    Possess,
    Necro{town_only: bool},
    Revive{town_only: bool},
    Transport,
    Wildcard,
    Reliant,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn transporter(game: &Game, actor_ref: PlayerReference) -> AttackData {
        AttackData {
            attack_type: AttackType::Transport,
            town_on_grave: actor_ref.town_on_grave(game),
        }
    }
    pub fn attack(game: &Game, actor_ref: PlayerReference, possess_immune: bool, transport_immune: bool) -> AttackData {
        AttackData {
            attack_type: AttackType::Attack{possess_immune, transport_immune},
            town_on_grave: actor_ref.town_on_grave(game),
        }
    }
    pub fn possess(game: &Game, actor_ref: PlayerReference) -> AttackData {
        AttackData {
            attack_type: AttackType::Possess,
            town_on_grave: actor_ref.town_on_grave(game)
        }
    }
    pub fn revive(game: &Game, actor_ref: PlayerReference, town_only: bool) -> AttackData {
        AttackData {
            attack_type: AttackType::Revive{town_only},
            town_on_grave: actor_ref.town_on_grave(game)
        }
    }
    pub fn necro(game: &Game, actor_ref: PlayerReference, town_only: bool) ->AttackData {
        AttackData {
            attack_type: AttackType::Necro{town_only},
            town_on_grave: actor_ref.town_on_grave(game)
        }
    }
    pub fn reliant(game: &Game, actor_ref: PlayerReference) -> AttackData {
        AttackData {
            attack_type: AttackType::Reliant,
            town_on_grave: actor_ref.town_on_grave(game)
        }
    }
    pub fn is_attack(&self) -> bool {
        match &self.attack_type {
            AttackType::Attack{..} => true,
            _ => false,
        }
    }
    pub fn is_none(&self) -> bool {
        match &self.attack_type {
            AttackType::None => true,
            _ => false,
        }
    }
    pub fn is_wildcard(&self) -> bool {
        match &self.attack_type {
            AttackType::Wildcard => true,
            _ => false,
        }
    }
}