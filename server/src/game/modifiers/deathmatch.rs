use std::collections::HashMap;

use crate::{game::{attack_type::{AttackData, AttackType}, chat::{ChatGroup, ChatMessageVariant}, game_conclusion::GameConclusion, phase::{PhaseState, PhaseType}, player::{self, PlayerReference}, role::Role, win_condition::{self, WinCondition}, Game}, vec_map::VecMap, vec_set::VecSet};

use super::{ModifierState, ModifierTrait, ModifierType, Modifiers};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Deathmatch{
    day_of_last_death: u8
}

/*
    There is modifier specific code in server.src\game\role\mod.rs
    in the defense function for role
*/
impl From<&Deathmatch> for ModifierType{
    fn from(_: &Deathmatch) -> Self {
        ModifierType::Deathmatch
    }
}

impl ModifierTrait for Deathmatch {
    fn on_game_start(self, game: &mut Game) {
        for player in PlayerReference::all_players(game){
            game.add_message_to_chat_group(
                ChatGroup::All, 
                ChatMessageVariant::PlayerHasWinCondition{player: player.index(), win_condition: player.win_condition(game).clone()}
            );
        }
    }
    fn on_phase_start(self, game: &mut Game, phase: PhaseState) {
        if phase.phase() == PhaseType::Nomination {
            game.on_fast_forward();
        }
    }
    fn on_any_death(self, game: &mut Game, _player:PlayerReference) {
        Modifiers::set_modifier(game, Deathmatch{day_of_last_death: game.day_number()}.into());
    }
}

impl Deathmatch {
    /// Should only be called on games that use the deathmatch modifier
    pub fn game_is_over(game: &Game) -> Option<GameConclusion>{
        //if nobody is left to hold game hostage
        if !PlayerReference::all_players(game)
            .any(|player| 
                if player.alive(game) {
                    player.keeps_game_running_deathmatch_quick(game)
                } else {
                    player.role(game) == Role::Jester
                }
        ) {
            return Some(GameConclusion::Draw);
        }
        //because a dead jester can keep the game running this has to be checked as well
        if PlayerReference::all_players(game).all(|p|!p.alive(game)){
            return Some(GameConclusion::Draw);
        }

        let Some(ModifierState::Deathmatch(deathmatch)) = Modifiers::get_modifier(game, ModifierType::Deathmatch) else {
            unreachable!("either Deathmatch::game_is_over has been called outside of a game with it enabled or the get modifier function managed not to return the right modifier.")
        };

        if deathmatch.day_of_last_death.saturating_add(5) >= game.day_number() {
            return GameConclusion::find_conclusion(game)
        } else if let Some(conclusion) = GameConclusion::find_conclusion(game) {
            return Some(conclusion)
        }

        let attack_data: HashMap<PlayerReference, AttackData> = HashMap::with_capacity(game.num_players() as usize);

        let conclusion_info: VecMap<GameConclusion, ConclusionData> = VecMap::new();

        let mut best_alive: VecSet<AttackData> = VecSet::new();
        let mut best_dead: VecSet<AttackData> = VecSet::new();

        for conclusion in GameConclusion::all() {
            conclusion_info.insert(conclusion, ConclusionData::default());
        }

        for player in PlayerReference::all_players(game) {
            let data = player.role_state(game).attack_data(game, player);
            attack_data.insert(player, data);

            match &data.attack_type {
                AttackType::None => (),
                AttackType::Attack { possess_immune, transport_immune } => {
                    match (possess_immune, transport_immune) {
                        (false, false) => 
                            if player.alive(game) {
                                best_alive.insert(data);
                            } else {
                                best_dead.insert( data);
                            }
                        (true, false) => 
                            if player.alive(game) {
                                if !best_alive.iter().any(|d|
                                    d.town_on_grave >= data.town_on_grave && 
                                    if let AttackType::Attack { possess_immune, transport_immune } = d.attack_type {
                                        possess_immune == true
                                    } else {
                                        false
                                    }
                                ) {
                                    for d in best_alive.clone() {
                                        if let AttackType::Attack { possess_immune, transport_immune } = d.attack_type {
                                            if  d.town_on_grave <= data.town_on_grave && transport_immune == false {
                                                best_alive.remove(&d);
                                            }
                                        }
                                    }
                                    best_alive.insert(data);
                                }
                            } else {
                                if !best_dead.iter().any(|d|
                                    d.town_on_grave >= data.town_on_grave && 
                                    if let AttackType::Attack { possess_immune, transport_immune } = d.attack_type {
                                        possess_immune == true
                                    } else {
                                        false
                                    }
                                ) {
                                    for d in best_dead.clone() {
                                        if let AttackType::Attack { possess_immune, transport_immune } = d.attack_type {
                                            if  d.town_on_grave <= data.town_on_grave && transport_immune == false {
                                                best_dead.remove(&d);
                                            }
                                        }
                                    }
                                    best_dead.insert(data);
                                }
                            }
                            
                    }
                }
            }
            match player.win_condition(game) {
                WinCondition::RoleStateWon => (),
                WinCondition::GameConclusionReached { win_if_any } => {
                    for condition in win_if_any {
                        let Some(data) = conclusion_info.get(condition) else {unreachable!("Conclusion that is not in Conclusion::all()")};
                        data.insert(player);
                    }
                }
            }
        }
        
        















        return ;
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ConclusionData{
    pub alive_players: VecSet<PlayerReference>,
    pub dead_players: VecSet<PlayerReference>,
    
}

impl ConclusionData{
    pub fn insert(&mut self, player: PlayerReference){
        if player.alive(game) {
            self.alive_players.insert(player)
        } else {
            self.dead_players.insert(player)
        }
    }
}