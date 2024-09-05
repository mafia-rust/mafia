use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::resolution_state::ResolutionState;
use crate::game::grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::Game;
use super::jester::Jester;
use super::{RoleStateImpl, Role, RoleState};


#[derive(Debug, Clone, Serialize, Default)]
pub struct Politician{
    won: bool,
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 1;

impl RoleStateImpl for Politician {
    fn get_won_game(self, _game: &Game, _actor_ref: PlayerReference) -> bool {
        self.won
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        if self.should_suicide(game, actor_ref) {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        if self.should_suicide(game, actor_ref) {
            actor_ref.set_role(game, RoleState::Jester(Jester::default()));
        }
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
        if self.should_suicide(game, actor_ref){
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }
    fn on_game_ending(self, game: &mut Game, actor_ref: PlayerReference){
        if !actor_ref.alive(game) {return}

        let mut won = false;
        //kill all townies who won
        for player_ref in PlayerReference::all_players(game) {
            if
                player_ref.alive(game) && 
                player_ref.role(game).faction() == Faction::Town &&
                player_ref.get_won_game(game)
            {
                
                if player_ref.defense(game) < 3 {

                    let mut grave = Grave::from_player_lynch(game, player_ref);
                    if let GraveInformation::Normal {death_cause, ..} = &mut grave.information {
                        *death_cause = GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Politician)]);
                    }
                    player_ref.die(game, grave);
                }else{
                    player_ref.add_private_chat_message(game, ChatMessageVariant::YouSurvivedAttack);
                    actor_ref.add_private_chat_message(game, ChatMessageVariant::SomeoneSurvivedYourAttack);
                }
                won = true;
            }
        }

        if 
            won ||
            PlayerReference::all_players(game).filter(|p|p.alive(game))
                .all(|player_ref| player_ref.role(game) == Role::Politician)
        {
            //kill all politicians because they all won
            for player_ref in PlayerReference::all_players(game) {
                if
                    player_ref.alive(game) && 
                    player_ref.role(game) == Role::Politician
                {
                    player_ref.set_role_state(game, RoleState::Politician(Politician{won: true}));
                    player_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
                }
            }
        }
    }
}

pub fn is_town_remaining(game: &Game) -> bool {
    PlayerReference::all_players(game).any(|player|
        player.alive(game) && ResolutionState::requires_only_this_resolution_state(game, player, ResolutionState::Town)
    )
}

impl Politician {
    pub fn should_suicide(&self, game: &Game, actor_ref: PlayerReference) -> bool {
        !self.won && actor_ref.alive(game) && !is_town_remaining(game)
    }
}