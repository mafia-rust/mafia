use crate::{game::{
    attack_power::AttackPower, chat::ChatMessageVariant,
    grave::{GraveInformation, GraveKiller, GraveReference}, player::PlayerReference,
    role::Priority, Game
}, vec_set::VecSet};

impl Game {
    pub fn poison(&self)->&Poison{
        &self.poison
    }
    pub fn set_poison(&mut self, poison: Poison){
        self.poison = poison;
    }
}

#[derive(Default, Clone)]
pub struct Poison{
    poisons: Vec<PlayerPoison>
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerPoison{
    player: PlayerReference,
    attack_power: AttackPower,
    grave_killer: GraveKiller,
    attackers: VecSet<PlayerReference>,
    leave_death_note: bool,
    obscure: PoisonObscure,
}
impl PlayerPoison{
    pub fn new(
        player: PlayerReference,
        attack_power: AttackPower,
        grave_killer: GraveKiller,
        attackers: VecSet<PlayerReference>,
        leave_death_note: bool,
        obscure: PoisonObscure,
    )->Self{
        Self{
            player,
            attack_power,
            grave_killer,
            attackers,
            leave_death_note,
            obscure
        }
    }
}

#[derive(PartialEq)]
pub enum PoisonAlert {
    NoAlert,
    Alert
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoisonObscure {
    NotObscured,
    Obscured
}

impl Poison{
    /// run this at night
    pub fn poison_player(
        game: &mut Game,
        player: PlayerReference,
        attack_power: AttackPower,
        grave_killer: GraveKiller,
        attackers: VecSet<PlayerReference>,
        death_note: bool,
        alert: PoisonAlert,
        obscure: PoisonObscure,
    ){
        let mut poison = game.poison().clone();
        poison.poisons.push(PlayerPoison::new(
            player, attack_power, grave_killer, attackers, death_note, obscure
        ));

        if alert == PoisonAlert::Alert {
            for poison in poison.poisons.iter(){
                poison.player.push_night_message(game, ChatMessageVariant::YouArePoisoned);
            }
        }

        game.set_poison(poison);
    }
    pub fn on_night_priority(game: &mut Game, priority: Priority){
        if priority != Priority::Kill{ return; }

        let mut poison = game.poison().clone();

        for poison in poison.poisons.iter_mut(){
            Self::attack_poisoned_player(game, poison.clone());
        }
        poison.poisons.clear();
        
        game.set_poison(poison);
    }
    fn attack_poisoned_player(game: &mut Game, poison: PlayerPoison){
        poison.player.try_night_kill(
            &poison.attackers,
            game,
            poison.grave_killer,
            poison.attack_power,
            poison.leave_death_note
        );
    }
    pub fn on_grave_added(game: &mut Game, grave_ref: GraveReference) {
        for poison in game.poison().clone().poisons {
            if poison.obscure == PoisonObscure::Obscured && poison.player == grave_ref.deref(game).player {
                grave_ref.deref_mut(game).information = GraveInformation::Obscured;
            }
        }
    }
}