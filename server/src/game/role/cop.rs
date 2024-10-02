
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::resolution_state::ResolutionState;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleStateImpl};



#[derive(Clone, Debug, Default)]
pub struct Cop {
    target_protected_ref: Option<PlayerReference>,
    night_selection: <Cop as RoleStateImpl>::RoleActionChoice,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Cop {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() == 1 {return}

        match priority {
            Priority::Heal => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;

                target_ref.increase_defense_to(game, DefensePower::Protection);
                actor_ref.set_role_state(game, Cop {target_protected_ref: Some(target_ref), ..self});
            }
            Priority::Kill => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;

                let mut player_to_attack = None;


                if let Some(non_town_visitor) = PlayerReference::all_players(game)
                    .filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref &&
                        !other_player_ref.win_condition(game).requires_only_this_resolution_state(ResolutionState::Town) &&
                        other_player_ref.night_visits(game)
                            .iter()
                            .any(|v|v.target==target_ref)
                    ).collect::<Vec<PlayerReference>>()
                    .choose(&mut rand::thread_rng())
                    .copied(){
                    player_to_attack = Some(non_town_visitor);
                }else if let Some(town_visitor) = PlayerReference::all_players(game)
                    .filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref &&
                        other_player_ref.night_visits(game)
                            .iter()
                            .any(|v|v.target==target_ref)
                    ).collect::<Vec<PlayerReference>>()
                    .choose(&mut rand::thread_rng())
                    .copied(){
                    player_to_attack = Some(town_visitor)
                }

                if let Some(player_to_attack) = player_to_attack{
                    player_to_attack.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Cop), AttackPower::Basic, false);
                }
            }
            Priority::Investigative => {
                if let Some(target_protected_ref) = self.target_protected_ref {
                    if target_protected_ref.night_attacked(game){
                        
                        actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                        target_protected_ref.push_night_message(game, ChatMessageVariant::YouWereProtected);
                    }
                }
            }
            _ => {}
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.player, false){
            return
        }
        if game.day_number() == 1 {return}

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(&self.night_selection, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase != PhaseType::Night {return;}
        actor_ref.set_role_state(game, Cop {target_protected_ref: None, night_selection: <Cop as RoleStateImpl>::RoleActionChoice::default()});
    }
}
impl GetClientRoleState<ClientRoleState> for Cop {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}