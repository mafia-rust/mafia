
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::{Grave, GraveReference};
use crate::game::phase::{PhaseState, PhaseType};
use crate::game::player::PlayerReference;
use crate::game::role::RoleState;
use crate::game::role_list::Faction;
use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::jester::Jester;
use super::{Priority, Role, RoleStateImpl};


#[derive(Clone, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Provocateur {
    target: ProvocateurTarget,
}
#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub enum ProvocateurTarget{
    Target(PlayerReference),
    Won,
}
impl ProvocateurTarget {
    fn get_target(&self)->Option<PlayerReference>{
        if let Self::Target(p) = self {
            Some(*p)
        }else{
            None
        }
    }
}
impl Default for ProvocateurTarget {
    fn default() -> Self {
        Self::Won
    }
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;

impl RoleStateImpl for Provocateur {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {1}
    


    fn do_night_action(self, _game: &mut Game, _actor_ref: PlayerReference, _priority: Priority) {
    }
    fn can_night_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
        
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, _target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        vec![]
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, _game: &Game, _actor_ref: PlayerReference) -> bool {
        self.target == ProvocateurTarget::Won
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){

        if self.target == ProvocateurTarget::Won || !actor_ref.alive(game){
            return;
        }

        match *game.current_phase() {
            PhaseState::FinalWords { player_on_trial } => {
                if Some(player_on_trial) == self.target.get_target() {
                    game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::ProvocateurWon);
                    actor_ref.set_role_state(game, RoleState::Provocateur(Provocateur { target: ProvocateurTarget::Won }));
                }
            }
            PhaseState::Night => {
                if self.target == ProvocateurTarget::Won {
                    actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
                }
            },
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
            actor_ref.push_player_tag(game, *target, Tag::ProvocateurTarget);
            actor_ref.set_role_state(game, RoleState::Provocateur(Provocateur{target: ProvocateurTarget::Target(*target)}));
        }else{
            actor_ref.set_role(game, RoleState::Jester(Jester::default()))
        };
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        if Some(dead_player_ref) == self.target.get_target() && self.target != ProvocateurTarget::Won {
            actor_ref.set_role(game, RoleState::Jester(Jester::default()))
        }
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
