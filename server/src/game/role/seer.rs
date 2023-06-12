use serde::Serialize;

use crate::game::chat::ChatGroup;
use crate::game::chat::night_message::NightInformation;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl};

pub(super) const SUSPICIOUS: bool = false;
pub(super) const WITCHABLE: bool = true;
pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownInvestigative;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Seer;

impl RoleStateImpl for Seer {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}

        if priority != Priority::Investigative {return;}

        let Some(first_visit) = actor_ref.night_visits(game).get(0) else {return;};
        let Some(second_visit) = actor_ref.night_visits(game).get(1) else {return;};

            
        if first_visit.target.night_jailed(game) || second_visit.target.night_jailed(game){
            actor_ref.push_night_message(game, NightInformation::TargetJailed);
            return
        }

        let message = NightInformation::SeerResult{enemies:
            !EndGameCondition::can_win_together(
                first_visit.target.night_appeared_role(game), 
                second_visit.target.night_appeared_role(game)
            )
        };
        
        actor_ref.push_night_message(game, message);
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {}
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        (
            actor_ref.chosen_targets(game).is_empty() || 
            actor_ref.chosen_targets(game).len() == 1 && *actor_ref.chosen_targets(game).get(0).unwrap() != target_ref
        )
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{ target: target_refs[0], astral:false, attack:false },
                Visit{ target: target_refs[1], astral:false, attack:false }
            ]
        } else {
            Vec::new()
        }
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType) {}
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
}