
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
use super::{common_role, CustomClientRoleState, Priority, Role, RoleState, RoleStateImpl};



#[derive(Clone, Debug, Default)]
pub struct Cop {
    target_protected_ref: Option<PlayerReference>
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState;

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl<ClientRoleState> for Cop {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if game.day_number() == 1 {return}

        match priority {
            Priority::Heal => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;

                target_ref.increase_defense_to(game, DefensePower::Protection);
                actor_ref.set_role_state(game, RoleState::Cop(Cop {target_protected_ref: Some(target_ref)}));
            }
            Priority::Kill => {
                let Some(visit) = actor_ref.night_visits(game).first() else {return};
                let target_ref = visit.target;

                let mut player_to_attack = None;


                if let Some(non_town_visitor) = PlayerReference::all_players(game)
                    .filter(|other_player_ref|
                        other_player_ref.alive(game) &&
                        *other_player_ref != actor_ref &&
                        !ResolutionState::requires_only_this_resolution_state(game, *other_player_ref, ResolutionState::Town) &&
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
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        common_role::can_night_select(game, actor_ref, target_ref) && game.day_number() > 1
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase != PhaseType::Night {return;}
        actor_ref.set_role_state(game, RoleState::Cop(Cop {target_protected_ref: None}));
    }
}

impl CustomClientRoleState<ClientRoleState> for Cop {
    fn get_client_role_state(self, _: &Game, _: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}