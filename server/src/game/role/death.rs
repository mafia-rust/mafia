use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::visit::Visit;
use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Default)]
pub struct Death{
    souls: u8,
    won: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState {
    souls: u8
}

const NEEDED_SOULS: u8 = 6;

pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Death {
    type ClientRoleState = ClientRoleState;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority == Priority::Heal && self.souls >= NEEDED_SOULS{
            actor_ref.set_night_upgraded_defense(game, Some(DefensePower::Invincible))
        }

        if priority != Priority::Investigative {return;}
        if !actor_ref.alive(game) {return;}

        let mut souls_to_gain = 1;

        if !crate::game::components::detained::Detained::is_detained(game, actor_ref) {
            let actor_visits = actor_ref.untagged_night_visits_cloned(game);
            if let Some(visit) = actor_visits.first(){
                let target_ref = visit.target;
                if target_ref.night_died(game) {
                    souls_to_gain = 2
                }
            }
        }

        self.souls += souls_to_gain;
        if self.souls >= NEEDED_SOULS {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::DeathCollectedSouls);
        }
        actor_ref.set_role_state(game, RoleState::Death(self));
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        match phase {
            PhaseType::Night => {
                if self.souls >= NEEDED_SOULS {
                    for player in PlayerReference::all_players(game){
                        if !player.alive(game){continue;}
                        if player.defense(game).can_block(AttackPower::ProtectionPiercing) {
                            player.add_private_chat_message(game, ChatMessageVariant::YouSurvivedAttack);
                            actor_ref.add_private_chat_message(game, ChatMessageVariant::SomeoneSurvivedYourAttack);
                
                        }else{
                            let mut grave = Grave::from_player_lynch(game, player);
                            if let GraveInformation::Normal{ death_cause, .. } = &mut grave.information {
                                *death_cause = GraveDeathCause::Killers(vec![GraveKiller::Role(Role::Death)]);
                            }
                            player.die(game, grave);
                            actor_ref.set_role_state(game, RoleState::Death(Death{won: true, souls: self.souls}));
                        }
                    }
                }
            },
            _=>{}
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Death {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState{
            souls: self.souls
        }
    }
}
impl Death {
    pub fn won(&self) -> bool {
        self.won
    }
}