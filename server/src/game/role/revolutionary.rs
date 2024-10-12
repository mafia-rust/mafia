
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::Grave;
use crate::game::phase::{PhaseState, PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role::RoleState;
use crate::game::role_list::Faction;
use crate::game::tag::Tag;

use crate::game::Game;
use super::jester::Jester;
use super::{GetClientRoleState, Role, RoleStateImpl};


#[derive(Clone, Debug, Default)]
pub struct Revolutionary {
    target: RevolutionaryTarget,
}

#[derive(Clone, Serialize, Debug)]
pub struct ClientRoleState;

#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub enum RevolutionaryTarget{
    Target(PlayerReference),
    Won,
}
impl RevolutionaryTarget {
    fn get_target(&self)->Option<PlayerReference>{
        if let Self::Target(p) = self {
            Some(*p)
        }else{
            None
        }
    }
}
impl Default for RevolutionaryTarget {
    fn default() -> Self {
        Self::Won
    }
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Revolutionary {
    type ClientRoleState = ClientRoleState;
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){

        if self.target == RevolutionaryTarget::Won || !actor_ref.alive(game){
            return;
        }

        match *game.current_phase() {
            PhaseState::FinalWords { player_on_trial } => {
                if Some(player_on_trial) == self.target.get_target() {
                    game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::RevolutionaryWon);
                    actor_ref.set_role_state(game, RoleState::Revolutionary(Revolutionary { target: RevolutionaryTarget::Won }));
                    actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
                }
            }
            _=>{}
        }
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        
        if let Some(target) = PlayerReference::all_players(game)
            .filter(|p|
                p.role(game).faction() == Faction::Town &&
                
                p.role(game) != Role::Jailor &&

                p.role(game) != Role::Deputy &&
                p.role(game) != Role::Veteran &&

                p.role(game) != Role::Transporter &&
                p.role(game) != Role::Mayor &&
                p.role(game) != Role::Journalist
            ).collect::<Vec<PlayerReference>>()
            .choose(&mut rand::thread_rng())
        {
            actor_ref.push_player_tag(game, *target, Tag::RevolutionaryTarget);
            actor_ref.set_role_state(game, RoleState::Revolutionary(Revolutionary{target: RevolutionaryTarget::Target(*target)}));
            actor_ref.insert_role_label(game, *target);
        }else{
            actor_ref.set_role_and_wincon(game, RoleState::Jester(Jester::default()))
        };
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        if Some(dead_player_ref) == self.target.get_target() && self.target != RevolutionaryTarget::Won {
            actor_ref.set_role_and_wincon(game, RoleState::Jester(Jester::default()))
        }
    }
}
impl GetClientRoleState<ClientRoleState> for Revolutionary {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}

impl Revolutionary {
    pub fn won(&self)->bool{
        self.target == RevolutionaryTarget::Won
    }
}