use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;

use crate::game::visit::Visit;
use crate::game::Game;
use super::{GetClientRoleState, Priority, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Default)]
pub struct Death{
    souls: u8,
    won: bool,
    night_selection: <Self as RoleStateImpl>::RoleActionChoice
}

#[derive(Clone, Debug, Serialize)]
pub struct ClientRoleState {
    souls: u8,
    night_selection: <Death as RoleStateImpl>::RoleActionChoice
}

const NEEDED_SOULS: u8 = 6;
pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Death {
    type ClientRoleState = ClientRoleState;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority == Priority::Heal && self.souls >= NEEDED_SOULS{
            actor_ref.set_night_upgraded_defense(game, Some(DefensePower::Invincible))
        }

        if priority != Priority::Investigative {return;}
        if !actor_ref.alive(game) {return;}

        let mut souls_to_gain = 1;

        if !actor_ref.night_jailed(game) {
            if let Some(visit) = actor_ref.night_visits(game).first(){
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
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.player, false){
            return
        }

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(&self.night_selection, false)
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
                            actor_ref.set_role_state(game, Death{won: true, souls: self.souls, night_selection: <Self as RoleStateImpl>::RoleActionChoice::default()});
                        }
                    }
                }
            },
            PhaseType::Obituary => {
                actor_ref.set_role_state(game, Death{night_selection: <Self as RoleStateImpl>::RoleActionChoice::default(), ..self});
            },
            _=>{}
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Death {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState{
            souls: self.souls,
            night_selection: self.night_selection
        }
    }
}
impl Death {
    pub fn won(&self) -> bool {
        self.won
    }
}