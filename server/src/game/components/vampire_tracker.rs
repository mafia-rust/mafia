use crate::{game::{attack_power::AttackPower, chat::ChatMessageVariant, event::on_fast_forward::OnFastForward, game_conclusion::GameConclusion, grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller, GravePhase}, phase::PhaseState, player::PlayerReference, role::{Priority, Role}, win_condition::WinCondition, Game}, vec_set::VecSet};


#[derive(Clone, Debug, Default)]

pub struct VampireTracker {
    //not using VecSet here because the order matters for tracking who has been a vampire the longest
    vampires: Vec<PlayerReference>,
    new_vampires: VecSet<PlayerReference>,
    max_converts: MaxConverts,
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
        let vampire_tracker = Self::vampire_data_mut(game);
        if vampire_tracker.vampires.contains(&player) {
            return false;
        }
        vampire_tracker.vampires.push(player);
        return true;
    }

    /// Returns true if the player is tracked
    pub fn is_tracked(game: &Game, player: PlayerReference) -> bool {
        let vampire_tracker = Self::vampire_data(game);
        return vampire_tracker.vampires.contains(&player);
    }
    
    pub fn max_converts(game: &Game) -> u8 {
        let vampire_tracker = Self::vampire_data(game);
        return vampire_tracker.max_converts.value();
    }

    /// Returns true if the player was already in vampires false otherwise
    pub fn remove(game: &mut Game, player: PlayerReference) -> bool {
        let vampire_tracker = Self::vampire_data_mut(game);
        vampire_tracker.new_vampires.remove(&player);
        let Some(index) = vampire_tracker.vampires.iter().position(|other|*other==player) else {return false};
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

    pub fn on_night_priority(game: &mut Game, priority: Priority){
        
        if priority != Priority::Kill {return};
        if game.day_number() == 1 {return;}
        let vampire_tracker = game.vampire_tracker.clone();      
        let mut new_vamp_que = vampire_tracker.new_vampires.clone();
        let converts = vampire_tracker.max_converts.value() as usize;

        for i in 0..vampire_tracker.vampires.len() {
            let vamp = vampire_tracker.vampires[i];
            let Some(visit) = vamp.untagged_night_visits_cloned(game).first().copied() else {continue};
            if converts <= i  {
                let target_ref = visit.target;
                if target_ref.night_defense(game).can_block(AttackPower::ArmorPiercing) || target_ref.role(&game) == Role::Vampire {
                    vamp.push_night_message(game, ChatMessageVariant::YourConvertFailed);
                    continue
                }
                target_ref.set_win_condition(
                    game,
                    WinCondition::new_loyalist(GameConclusion::Fiends)
                );
                target_ref.set_night_convert_role_to(game, Some(Role::Vampire.new_state(game)));
                new_vamp_que.insert(target_ref);
            } else if game.day_number() != 1 && !vampire_tracker.new_vampires.contains(&visit.target) {
                visit.target.try_night_kill_single_attacker(
                    vamp,
                    game,
                    GraveKiller::Role(Role::Vampire),
                    AttackPower::ArmorPiercing,
                    true
                );
            }
        }

        game.vampire_tracker.new_vampires = game.vampire_tracker.new_vampires.union(&new_vamp_que);
    }

    pub fn on_phase_start(game: &mut Game, phase: PhaseState){
        match phase {
            PhaseState::Obituary => {
                let vampire_tracker = game.vampire_tracker.clone();
                let mut new_vampires: Vec<PlayerReference> = vampire_tracker.new_vampires.into();
                new_vampires.retain(|vamp| vamp.alive(game) && !vampire_tracker.vampires.clone().contains(vamp));

                let new_vamp_message = ChatMessageVariant::NewVampires { vampires: new_vampires.clone() };
                
                for vamp in new_vampires.clone() {
                    Self::track(game, vamp);
                }

                for vamp in vampire_tracker.vampires.clone() {
                    vamp.add_private_chat_message(game, new_vamp_message.clone());
                }

                for vamp in new_vampires {
                    let mut other_vampires = vampire_tracker.vampires.clone();
                    other_vampires.retain(|other_vamp| vamp != *other_vamp);
                    vamp.add_private_chat_message(game, ChatMessageVariant::CurrentVampires { 
                        vampires: other_vampires
                    });
                }

                game.vampire_tracker = VampireTracker {
                    vampires: vampire_tracker.vampires,
                    new_vampires: VecSet::with_capacity(vampire_tracker.max_converts.value() as usize),
                    max_converts: vampire_tracker.max_converts,
                };
            },

            PhaseState::Judgement{player_on_trial,..} => {
                if game.day_number() <= 2 {return;}
                if player_on_trial.role(game) != Role::Vampire {return}
                OnFastForward::invoke(game);
            }

            PhaseState::Night => {
                if game.day_number() == 1 {return;}
                let vampire_tracker = game.vampire_tracker.clone();
                for i in 0..vampire_tracker.vampires.len().min(vampire_tracker.max_converts.value() as usize){
                    let vamp = vampire_tracker.vampires[i];
                    vamp.add_private_chat_message(game, ChatMessageVariant::VampireCanConvert);
                }
            }

            _=>(),
        }    
    }

    pub fn before_phase_end(game: &mut Game, phase: PhaseState){
        let PhaseState::Judgement {player_on_trial,.. } = phase else {return};
        if player_on_trial.role(game) != Role::Vampire {return}
        player_on_trial.set_alive(game, false);
        
        player_on_trial.die(game, Grave{
            day_number: game.day_number(),
            died_phase: GravePhase::Day,
            information: GraveInformation::Normal { 
                role: Role::Vampire, 
                will: player_on_trial.will(game).to_owned(), 
                death_cause: GraveDeathCause::Killers(vec![GraveKiller::Sun]), 
                death_notes: vec![],
            },
            player: player_on_trial,
        });
    }
}