use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::{GraveKiller, Grave, GraveDeathCause};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::jester::Jester;
use super::{Priority, RoleStateImpl, Role, RoleState};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Politician{
    won: bool,
}

pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::NeutralEvil;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Politician {
    fn suspicious(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {true}
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {1}
    fn control_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn roleblock_immune(&self, _game: &Game, _actor_ref: PlayerReference) -> bool {false}
    fn end_game_condition(&self, _game: &Game, _actor_ref: PlayerReference) -> EndGameCondition {EndGameCondition::None}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {None}


    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {

    }
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, _game: &Game, _actor_ref: PlayerReference) -> bool {
        self.won
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        if self.should_suicide(game, actor_ref) {
            actor_ref.die(game, Grave::from_player_suicide(game, actor_ref));
        }
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        if self.should_suicide(game, actor_ref) {
            actor_ref.set_role(game, RoleState::Jester(Jester::default()));
        }
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
        if self.should_suicide(game, actor_ref) {
            actor_ref.die(game, Grave::from_player_suicide(game, actor_ref));
        }
    }
    fn on_game_ending(self, game: &mut Game, actor_ref: PlayerReference){
        if !actor_ref.alive(game) {return}

        let mut won = false;
        //kill all townies who won
        for player_ref in PlayerReference::all_players(game) {
            if
                player_ref.alive(game) && 
                player_ref.role(game).faction_alignment().faction() == Faction::Town &&
                player_ref.get_won_game(game)
            {
                
                if player_ref.defense(game) >= 3 {

                    let mut grave = Grave::from_player_lynch(game, player_ref);
                    grave.death_cause = GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Politician)]);
                    player_ref.die(game, grave);
                }else{
                    player_ref.add_chat_message(game, ChatMessage::YouSurvivedAttack);
                    actor_ref.add_chat_message(game, ChatMessage::TargetSurvivedAttack);
                }
                won = true;
            }
        }

        if won {
            //kill all politicians because they all won
            for player_ref in PlayerReference::all_players(game) {
                if
                    player_ref.alive(game) && 
                    player_ref.role(game) == Role::Politician
                {
                    player_ref.set_role_state(game, RoleState::Politician(Politician{won: true}));

                    if player_ref.defense(game) >= 3 {
                        player_ref.die(game, Grave::from_player_suicide(game, player_ref));
                    }else{
                        player_ref.add_chat_message(game, ChatMessage::YouSurvivedAttack);
                        actor_ref.add_chat_message(game, ChatMessage::TargetSurvivedAttack);
                    }
                }
            }
        }
    }
}

pub fn is_town_remaining(game: &Game) -> bool {
    PlayerReference::all_players(game).into_iter().any(|player|
        player.alive(game) && player.role(game).faction_alignment().faction() == Faction::Town
    )
}

impl Politician {
    pub fn should_suicide(&self, game: &Game, actor_ref: PlayerReference) -> bool {
        !self.won && actor_ref.alive(game) && !is_town_remaining(game)
    }
}