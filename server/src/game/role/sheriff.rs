use serde::{Deserialize, Serialize};

use crate::game::chat::ChatGroup;
use crate::game::chat::night_message::NightInformation;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleFunctions};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Sheriff;
impl RoleFunctions for Sheriff {
    fn suspicious() -> bool { false }
    fn witchable() -> bool { true }
    fn defense() -> u8 {0}
    fn roleblockable() -> bool {true}
    fn faction_alignment() -> FactionAlignment { FactionAlignment::TownInvestigative }
    fn maximum_count() -> Option<u8> { None }
    fn end_game_condition() -> EndGameCondition { EndGameCondition::Faction }
    fn team() -> Option<Team> { None }

    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}

        if priority != Priority::Investigative {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            
            if visit.target.night_jailed(game){
                actor_ref.push_night_messages(game, NightInformation::TargetJailed );
                return
            }
            let message = NightInformation::SheriffResult { suspicious: visit.target.night_appeared_suspicious(game)};
            
            actor_ref.push_night_messages(game, message);
        }
    }
    fn do_day_action(self, game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {}
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref)
    }
    fn can_day_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
    }
    fn get_current_send_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self,  game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self,  game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {}
    fn on_role_creation(self,  game: &mut Game, actor_ref: PlayerReference) {
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
}