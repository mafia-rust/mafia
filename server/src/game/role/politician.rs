use serde::Serialize;

use crate::game::chat::ChatGroup;
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
        self.check_if_no_town_and_leave_town(game, actor_ref);
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        self.check_if_no_town_and_convert_to_jester(game, actor_ref);
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
        self.check_if_no_town_and_leave_town(game, actor_ref);
    }
    fn on_game_ending(self, game: &mut Game, actor_ref: PlayerReference){
        Politician::check_if_town_wouldve_won_and_kill(game, actor_ref);
    }
}

impl Politician{
    pub fn check_if_no_town_and_convert_to_jester(&self, game: &mut Game, actor_ref: PlayerReference){
        if
            !self.won &&
            actor_ref.alive(game) &&
            PlayerReference::all_players(game).into_iter().filter(|player|
                player.alive(game) && player.role(game).faction_alignment().faction() == Faction::Town
            ).collect::<Vec<PlayerReference>>().len() == 0
        {
            actor_ref.set_role(game, RoleState::Jester(Jester::default()));
        }
    }
    pub fn check_if_no_town_and_leave_town(&self, game: &mut Game, actor_ref: PlayerReference){
        if
            !self.won &&
            actor_ref.alive(game) &&
            PlayerReference::all_players(game).into_iter().filter(|player|
                player.alive(game) && player.role(game).faction_alignment().faction() == Faction::Town
            ).collect::<Vec<PlayerReference>>().len() == 0
        {
            let mut grave = Grave::from_player_lynch(game, actor_ref);
            grave.death_cause = GraveDeathCause::Killers(vec![GraveKiller::Suicide]);
            actor_ref.die(game, grave);
        }
    }
    pub fn check_if_town_wouldve_won_and_kill(game: &mut Game, actor_ref: PlayerReference) {
        if !actor_ref.alive(game) {return}

        let mut won = false;
        for player_ref in PlayerReference::all_players(game) {
            if
                player_ref.alive(game) && 
                player_ref.role(game).faction_alignment().faction() == Faction::Town &&
                player_ref.get_won_game(game)
            {
                let mut grave = Grave::from_player_lynch(game, player_ref);
                grave.death_cause = GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Politician)]);
                player_ref.die(game, grave);
                won = true;
            }
        }

        if won {
            for player_ref in PlayerReference::all_players(game) {
                if
                    player_ref.alive(game) && 
                    player_ref.role(game) == Role::Politician
                {
                    player_ref.set_role_state(game, RoleState::Politician(Politician{won: true}));

                    let mut grave = Grave::from_player_lynch(game, player_ref);
                    grave.death_cause = GraveDeathCause::Killers(vec![GraveKiller::Suicide]);
                    player_ref.die(game, grave);
                }
            }
        }
    }
}