use serde::Serialize;

use crate::game::player_group::PlayerGroup;
use crate::game::grave::{GraveKiller, GraveReference};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleState, Role, RoleStateImpl};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Veteran { 
    alerts_remaining: u8, 
    alerting_tonight: bool 
}

impl Default for Veteran {
    fn default() -> Self {
        Veteran {
            alerts_remaining: 3,
            alerting_tonight: false
        }
    }
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Veteran {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::TopPriority => {
                if self.alerts_remaining > 0 && game.day_number() > 1{
                    if let Some(selection) = actor_ref.selection(game).first(){
                        if *selection == actor_ref{
                            actor_ref.set_role_state(game, RoleState::Veteran(Veteran { 
                                alerts_remaining: self.alerts_remaining - 1, 
                                alerting_tonight: true 
                            }));
                        }
                    }
                }
            }
            Priority::Heal=>{
                if !self.alerting_tonight {return}
                actor_ref.increase_defense_to(game, 2);
            }
            Priority::Kill => {
                if !self.alerting_tonight {return}

                for other_player_ref in actor_ref.all_visitors(game)
                    .into_iter().filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref
                    ).collect::<Vec<PlayerReference>>()
                {
                    other_player_ref.try_night_kill(actor_ref, game, GraveKiller::Role(Role::Veteran), 2, false);
                }
            }
            _=>{}
        }
    }
    
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref == target_ref &&
        !actor_ref.night_jailed(game) &&
        self.alerts_remaining > 0 &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game) &&
        game.day_number() > 1
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {

    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<PlayerGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, RoleState::Veteran(Veteran { alerts_remaining: self.alerts_remaining, alerting_tonight: false }));   
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
        
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}