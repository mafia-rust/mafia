use serde::{Deserialize, Serialize};

use crate::game::{attack_power::DefensePower, player::PlayerReference};
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::common_role::{
    convert_action_choice_to_visits,
    convert_action_choice_to_visits_two_players,
    default_action_choice_one_player_is_valid,
    default_action_choice_two_players_is_valid
};
use super::{Priority, RoleStateImpl};


#[derive(Clone, Debug, Default, Serialize)]
pub struct Framer{
    pub target: FrameTarget
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    frame: FrameTarget
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum FrameTarget{
    #[default]
    None,
    Aura{
        target: PlayerReference
    },
    AuraAndVisit{
        target: PlayerReference,
        visit: PlayerReference
    },
}


pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Framer {
    type ClientRoleState = Framer;
    type RoleActionChoice = RoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Deception {return;}
    
        let framer_visits = actor_ref.night_visits(game).clone();


        let Some(first_visit) = framer_visits.first() else {return};

        first_visit.target.set_night_framed(game, true);

        let Some(second_visit) = framer_visits.get(1) else {return};
    
        if !first_visit.target.night_jailed(game) {
            first_visit.target.set_night_appeared_visits(game, Some(vec![
                Visit{ target: second_visit.target, attack: false }
            ]));
        }

        actor_ref.set_night_visits(game, vec![first_visit.clone()]);
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};

        match action_choice.frame {
            FrameTarget::Aura{ target } => {
                if default_action_choice_one_player_is_valid(game, actor_ref, Some(target), false){
                    self.target = action_choice.frame;
                }
            },
            FrameTarget::AuraAndVisit{ target, visit } => {
                if default_action_choice_two_players_is_valid(game, actor_ref, Some((target,visit)), (false, true), true) {
                    self.target = action_choice.frame;
                }
        
            },
            FrameTarget::None => {
                self.target = action_choice.frame;
            }
        }
        actor_ref.set_role_state(game, self);    
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        match self.target {
            FrameTarget::Aura { target } => {
                convert_action_choice_to_visits(Some(target), false)
                
            },
            FrameTarget::AuraAndVisit { target, visit } => {
                convert_action_choice_to_visits_two_players(Some((target, visit)), true)
            },
            FrameTarget::None => {
                vec![]
            },
        }
    }
}
