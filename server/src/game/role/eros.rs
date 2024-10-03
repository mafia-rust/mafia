use serde::{Deserialize, Serialize};

use crate::game::attack_power::AttackPower;
use crate::game::phase::PhaseType;
use crate::game::{attack_power::DefensePower, components::love_linked::LoveLinked};
use crate::game::grave::GraveKiller;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::common_role::{default_action_choice_one_player_is_valid, default_action_choice_two_players_is_valid};
use super::{Priority, RoleStateImpl};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Eros{
    pub action: ErosActionChoice,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleActionChoice{
    action: ErosActionChoice
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ErosActionChoice{
    #[default]
    None,
    SetAttack{target: PlayerReference},
    SetLoveLink{targets: (PlayerReference, PlayerReference)},
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Eros {
    type ClientRoleState = Eros;
    type RoleActionChoice = RoleActionChoice;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match (priority, self.action) {
            (Priority::Kill, ErosActionChoice::SetAttack{..}) => {
                if game.day_number() == 1 {return}
                if let Some(visit) = actor_ref.night_visits(game).first(){
                    let target_ref = visit.target;
            
                    target_ref.try_night_kill_single_attacker(
                        actor_ref, game, GraveKiller::Faction(Faction::Mafia), AttackPower::Basic, false
                    );
                }
            }
            (Priority::Cupid, ErosActionChoice::SetLoveLink{..}) => {
                let visits = actor_ref.night_visits(game);

                let Some(first_visit) = visits.get(0) else {return};
                let Some(second_visit) = visits.get(1) else {return};
                
                let player1 = first_visit.target;
                let player2 = second_visit.target;

                LoveLinked::add_love_link(game, player1, player2);
            },
            _ => ()
        }
    }
    fn on_role_action(self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != PhaseType::Night {return};
        match action_choice.action{
            ErosActionChoice::SetAttack { target } => {
                if game.day_number() > 1 && default_action_choice_one_player_is_valid(game, actor_ref, Some(target), false){
                    actor_ref.set_role_state(game, Eros{
                        action: action_choice.action,
                    });
                }else{
                    actor_ref.set_role_state(game, Eros{
                        action: ErosActionChoice::None,
                    });
                }

            },
            ErosActionChoice::SetLoveLink { targets } => {
                if default_action_choice_two_players_is_valid(game, actor_ref, Some(targets), (true, true), false) {
                    actor_ref.set_role_state(game, Eros{
                        action: action_choice.action,
                    });
                }else{
                    actor_ref.set_role_state(game, Eros{
                        action: ErosActionChoice::None,
                    });
                }
                
            },
            ErosActionChoice::None => {
                actor_ref.set_role_state(game, Eros{
                    action: action_choice.action,
                });
            },
        }
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        match self.action {
            ErosActionChoice::SetAttack { target } => {
                crate::game::role::common_role::convert_action_choice_to_visits(Some(target), true)
            },
            ErosActionChoice::SetLoveLink { targets } => {
                crate::game::role::common_role::convert_action_choice_to_visits_two_players(Some(targets), true)
            },
            ErosActionChoice::None => {
                vec![]
            },
        }
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: crate::game::phase::PhaseType) {
        actor_ref.set_role_state(game, Eros{
            action: ErosActionChoice::None,
        });
    }
}