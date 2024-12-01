
use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bodyguard {
    self_shields_remaining: u8,
    target_protected_ref: Option<PlayerReference>,
    redirected_player_refs: Vec<PlayerReference>
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleState {
    self_shields_remaining: u8
}

impl Default for Bodyguard {
    fn default() -> Self {
        Self { 
            self_shields_remaining: 1, 
            target_protected_ref: None, 
            redirected_player_refs: Vec::new()
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Bodyguard {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Bodyguard => {
                let actor_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actor_visits.first() else {return};
                let target_ref = visit.target;
                if actor_ref == target_ref {return}

                let mut redirected_player_refs = vec![];
                let mut target_protected_ref = None;
                for attacker_ref in PlayerReference::all_players(game){
                    let mut new_visits = vec![];
                    for mut attacking_visit in attacker_ref.untagged_night_visits_cloned(game).clone(){
                        if attacking_visit.target == target_ref && attacking_visit.attack {
                            attacking_visit.target = actor_ref;
                            redirected_player_refs.push(attacker_ref);
                            target_protected_ref = Some(target_ref);
                        }
                        new_visits.push(attacking_visit);
                    }
                    attacker_ref.set_night_visits(game, new_visits);
                }

                actor_ref.set_role_state(game, Bodyguard {
                    self_shields_remaining: self.self_shields_remaining, 
                    target_protected_ref, 
                    redirected_player_refs
                });
                
            },
            Priority::Heal => {
                let actors_visits = actor_ref.untagged_night_visits_cloned(game);
                let Some(visit) = actors_visits.first() else {return};
                let target_ref = visit.target;
    
                if actor_ref == target_ref {
                    let self_shields_remaining = self.self_shields_remaining - 1;
                    actor_ref.set_role_state(game, Bodyguard{
                        self_shields_remaining, 
                        ..self
                    });
                    
                    
                    target_ref.increase_defense_to(game, DefensePower::Protection);
                }
            },
            Priority::Kill => {
                for redirected_player_ref in self.redirected_player_refs {
                    redirected_player_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Bodyguard), AttackPower::ArmorPiercing, false);
                }
            }
            Priority::Investigative => {
                if let Some(target_protected_ref) = self.target_protected_ref {
                    actor_ref.push_night_message(game, ChatMessageVariant::TargetWasAttacked);
                    target_protected_ref.push_night_message(game, ChatMessageVariant::YouWereProtected);
                }
            }
            _ => {}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        game.day_number() > 1 &&
        (actor_ref != target_ref || self.self_shields_remaining > 0) &&
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game) &&
        target_ref.alive(game)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        let redirected_player_refs = Vec::new();
        let target_protected_ref = None;
        actor_ref.set_role_state(game, RoleState::Bodyguard(Bodyguard { self_shields_remaining: self.self_shields_remaining, redirected_player_refs, target_protected_ref }));
    }
}
impl GetClientRoleState<ClientRoleState> for Bodyguard {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState {
            self_shields_remaining: self.self_shields_remaining
        }
    }
}