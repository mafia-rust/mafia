use serde::Serialize;

use crate::game::phase::PhaseType;
use crate::game::{attack_power::DefensePower, components::arsonist_doused::ArsonistDoused};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{Priority, RoleStateImpl, Role};


#[derive(Clone, Debug, Serialize, Default)]
pub struct Arsonist{
    night_selection: <Self as RoleStateImpl>::RoleActionChoice,
}

pub(super) const FACTION: Faction = Faction::Fiends;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Arsonist {
    type ClientRoleState = Self;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        

        match priority {
            Priority::Deception => {
                if !actor_ref.night_jailed(game) {
                    //douse target
                    if let Some(visit) = actor_ref.night_visits(game).first(){
                        let target_ref = visit.target;
                        ArsonistDoused::douse(game, target_ref);
                    }

                    
                }else{
                    //douse the jailor if jailed
                    for player_ref in PlayerReference::all_players(game){
                        if player_ref.alive(game) && player_ref.role(game) == Role::Jailor {
                            ArsonistDoused::douse(game, player_ref);
                        }
                    }
                }
                
                //douse all visitors
                for other_player_ref in PlayerReference::all_players(game)
                    .filter(|other_player_ref|
                        *other_player_ref != actor_ref &&
                        other_player_ref.night_visits(game)
                            .iter()
                            .any(|v|v.target==actor_ref)
                    ).collect::<Vec<PlayerReference>>()
                {   
                    ArsonistDoused::douse(game, other_player_ref);
                }
            },
            Priority::Kill => {
                if actor_ref.night_jailed(game) {return}
                
                if let Some(visit) = actor_ref.night_visits(game).first(){
                    if actor_ref == visit.target{
                        ArsonistDoused::ignite(game, actor_ref);
                    }
                }
                
            }
            _ => {}
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.player, true) {return}
        
        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(&self.night_selection,false)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        crate::on_phase_start_reset_night_selection!(self, game, actor_ref, phase);
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        ArsonistDoused::clean_doused(game, actor_ref);
    }
}
