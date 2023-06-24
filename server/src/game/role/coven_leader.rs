use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::tag::Tag;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl, RoleState};

pub(super) const DEFENSE: u8 = 1;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::CovenPower;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = Some(Team::Coven);

#[derive(Clone, Debug, Default, Serialize)]
pub struct CovenLeader {
    pub necronomicon: bool
}

impl RoleStateImpl for CovenLeader {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        //TODO NECRONOMICON
        
        if actor_ref.night_jailed(game) {return;}
    
        if priority != Priority::Kill {return}
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;
            if target_ref.night_jailed(game) {
                actor_ref.push_night_message(game, ChatMessage::TargetJailed)
            }else {
                target_ref.try_night_kill(actor_ref, game, GraveKiller::Faction(Faction::Coven), 1);
            }
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Coven])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase != PhaseType::Night {return;}
    
        let should_get_necronomicon = actor_ref.alive(game) && !PlayerReference::all_players(game).into_iter()
            .any(|p|p.role_state(game).has_necronomicon() && p.alive(game));
        
        if should_get_necronomicon {
            actor_ref.set_role_state(game, RoleState::CovenLeader(Self { necronomicon: true }));
            for player_ref in PlayerReference::all_players(game) {
                if player_ref.role(game).faction_alignment().faction() == Faction::Coven{
                    player_ref.push_player_tag(game, actor_ref, Tag::Necronomicon);
                    player_ref.add_chat_message(game, ChatMessage::PlayerWithNecronomicon{ player_index: actor_ref.index() });
                }
            }
        }
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
}
