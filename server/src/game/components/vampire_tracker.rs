use rand::seq::SliceRandom;

use crate::{game::{attack_power::AttackPower, chat::ChatMessageVariant, event::on_fast_forward::OnFastForward, game_conclusion::GameConclusion, grave::{Grave, GraveKiller}, phase::{PhaseState, PhaseType}, player::PlayerReference, role::{Priority, Role}, win_condition::WinCondition, Game}, vec_set::VecSet};


#[derive(Clone, Debug, Default)]

pub struct VampireTracker {
    //not using VecSet here because the order matters for tracking who has been a vampire the longest
    vampires: Vec<(PlayerReference, u8)>,
    new_vampires: VecSet<PlayerReference>,
    max_converts: MaxConverts,
    vamp_on_trial: Option<PlayerReference>,
}

/// Its like this because otherwise if a player is converted to a vampire and there were no vampires to begin with
/// then the vampires can't convert anyone
/// But if I had it default to 1 then i can't tell the difference between there was 1 vampire to begin with 
/// and no vampires to begin with
/// Also wildcards that choose vampires are considered to considered to be players that started out as vampires
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
enum MaxConverts {
    #[default]
    OneNoInitialVamps,
    Normal(u8),
}

impl MaxConverts {
    pub fn increment(&self) -> Self {
        match self {
            Self::Normal(val) => Self::Normal(val.saturating_add(1)),
            Self::OneNoInitialVamps => Self::Normal(1),
        }
    }
    pub fn value(&self) -> u8 {
        match self {
            Self::Normal(val) => *val,
            Self::OneNoInitialVamps => 1,
        }
    }
}

impl VampireTracker {
    pub fn vampire_data<'a>(game: &'a Game)->&'a Self{
        return &game.vampire_tracker
    }
    pub fn vampire_data_mut<'a>(game: &'a mut Game)->&'a mut Self{
        return &mut game.vampire_tracker
    }
    
    /// Returns true if the player was not already tracked, and false otherwise
    pub fn track(game: &mut Game, player: PlayerReference) -> bool{
        if Self::is_tracked(game, player) {return false}
        let day = game.day_number();
        let vampire_tracker = Self::vampire_data_mut(game);
        vampire_tracker.vampires.push((player, day));
        return true;
    }

    /// Returns true if the player is tracked
    pub fn is_tracked(game: &Game, player: PlayerReference) -> bool {
        let vampire_tracker = Self::vampire_data(game);
        return vampire_tracker.vampires.iter().any(|v|v.0==player);
    }
    
    pub fn max_converts(game: &Game) -> u8 {
        let vampire_tracker = Self::vampire_data(game);
        return vampire_tracker.max_converts.value();
    }

    /// Returns true if the player was already in vampires false otherwise
    pub fn remove(game: &mut Game, player: PlayerReference) -> bool {
        let vampire_tracker = Self::vampire_data_mut(game);
        vampire_tracker.new_vampires.remove(&player);
        let Some(index) = vampire_tracker.vampires.iter().position(|other|other.0==player) else {return false};
        vampire_tracker.vampires.remove(index);
        return true;
    }

    pub fn on_any_death(game: &mut Game, dead_player: PlayerReference){
        Self::remove(game, dead_player);
    }

    pub fn before_role_switch(game: &mut Game, player: PlayerReference, new: Role, old: Role) {
        if new == old {return};
        if new == Role::Vampire {
            let vampire_tracker = Self::vampire_data_mut(game);
            if old == Role::Wildcard || old == Role::FiendsWildcard || old == Role::TrueWildcard {
                vampire_tracker.max_converts.increment();
            }
            vampire_tracker.new_vampires.insert(player);
        } else if old == Role::Vampire {
            Self::remove(game, player);
        }
    }  

    pub fn before_initial_role_creation(game: &mut Game, player: PlayerReference){
        let vampire_tracker = Self::vampire_data_mut(game);
        vampire_tracker.max_converts.increment();
        Self::track(game, player);
    }

    pub fn on_night_priority(game: &mut Game, priority: Priority){
        
        if priority != Priority::Kill {return};
        if game.day_number() == 1 {return;}
        let vampire_tracker = game.vampire_tracker.clone();      
        let mut new_vampires = vampire_tracker.new_vampires.clone();
        let converts = vampire_tracker.max_converts.value() as usize;

        for i in 0..vampire_tracker.vampires.len() {
            let vamp = vampire_tracker.vampires[i].0;
            let Some(visit) = vamp.untagged_night_visits_cloned(game).first().copied() else {continue};
            let target_ref = visit.target;
            if converts > i  {
                if target_ref.night_defense(game).can_block(AttackPower::ArmorPiercing) || target_ref.role(&game) == Role::Vampire {
                    vamp.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                    continue
                }
                target_ref.set_win_condition(
                    game,
                    WinCondition::new_loyalist(GameConclusion::Fiends)
                );
                target_ref.set_night_convert_role_to(game, Some(Role::Vampire.new_state(game)));
                new_vampires.insert(target_ref);
            } else if !new_vampires.contains(&target_ref) {
                target_ref.try_night_kill_single_attacker(
                    vamp,
                    game,
                    GraveKiller::Role(Role::Vampire),
                    AttackPower::ArmorPiercing,
                    true
                );
            }
        }

        game.vampire_tracker.new_vampires = game.vampire_tracker.new_vampires.union(&new_vampires);
    }

    pub fn on_phase_start(game: &mut Game, phase: PhaseState){
        match phase {
            PhaseState::Obituary => {
                let vampire_tracker = game.vampire_tracker.clone();
                let mut new_vampires: Vec<PlayerReference> = vampire_tracker.new_vampires.into();
                new_vampires.retain(|vamp| vamp.alive(game) && !Self::is_tracked(game, *vamp));

                if new_vampires.is_empty() {
                    game.vampire_tracker = VampireTracker {
                        vampires: vampire_tracker.vampires,
                        new_vampires: VecSet::with_capacity(vampire_tracker.max_converts.value() as usize),
                        max_converts: vampire_tracker.max_converts,
                        vamp_on_trial: None,
                    };
                    return;
                };
                new_vampires.shuffle(&mut rand::rng());

                let new_vamp_message = ChatMessageVariant::NewVampires { vampires: new_vampires.clone() };
                
                for vamp in new_vampires.clone() {
                    Self::track(game, vamp);
                }

                let vampires = game.vampire_tracker.vampires.clone();

                for vamp in vampires.clone() {
                    let mut other_vampires = vampires.clone();
                    other_vampires.retain(|other_vamp| vamp != *other_vamp);
                    vamp.0.add_private_chat_message(game, ChatMessageVariant::CurrentVampires { 
                        vampires: other_vampires.iter().map(|x|x.0).collect()
                    });
                    vamp.0.add_private_chat_message(game, new_vamp_message.clone());
                }

                game.vampire_tracker = VampireTracker {
                    vampires: vampires,
                    new_vampires: VecSet::with_capacity(vampire_tracker.max_converts.value() as usize),
                    max_converts: vampire_tracker.max_converts,
                    vamp_on_trial: None,
                };
            },

            PhaseState::Judgement{player_on_trial,..} => {
                if game.day_number() <= 2 {return;}
                if player_on_trial.role(game) != Role::Vampire {return}
                let Some(data) = game.vampire_tracker.vampires.iter().find(|x|x.0==player_on_trial) else {return};
                if data.1 == game.day_number() {return}
                
                OnFastForward::invoke(game);
                game.vampire_tracker.vamp_on_trial = Some(player_on_trial);
            }

            PhaseState::Night => {
                if game.day_number() == 1 {return;}
                let vampire_tracker = game.vampire_tracker.clone();
                for i in 0..vampire_tracker.vampires.len().min(vampire_tracker.max_converts.value() as usize){
                    let vamp = vampire_tracker.vampires[i].0;
                    vamp.add_private_chat_message(game, ChatMessageVariant::VampireCanConvert);
                }
            }

            _=>(),
        }    
    }

    pub fn before_phase_end(game: &mut Game, phase: PhaseType){
        if phase != PhaseType::Judgement {return};

        let Some(vamp_on_trial) = game.vampire_tracker.vamp_on_trial else {return};

        vamp_on_trial.set_alive(game, false);
        vamp_on_trial.die(game, Grave::from_sun(game, vamp_on_trial));

        game.vampire_tracker.vamp_on_trial = None;
    }
}