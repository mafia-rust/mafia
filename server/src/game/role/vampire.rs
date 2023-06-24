use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleStateImpl, RoleState, common_role};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::NeutralChaos;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(2);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::SingleRole;
pub(super) const TEAM: Option<Team> = Some(Team::Vampire);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vampire;

impl Default for Vampire {
    fn default() -> Self {
        Vampire {}
    }
}

impl RoleStateImpl for Vampire {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}
        if priority != Priority::Convert {return;}

        let Some(visit) = actor_ref.night_visits(game).first() else {return;};
        let target_ref = visit.target;

        if target_ref.night_jailed(game) {
            actor_ref.push_night_message(game, ChatMessage::TargetJailed);
            return
        }
        if target_ref.night_defense(game) >= 1 {
            actor_ref.push_night_message(game, ChatMessage::TargetSurvivedAttack);
            return
        }

        let mut vampires = game.teams.vampire().clone();
        vampires.night_last_converted = Some(game.phase_machine.day_number);
        game.teams.set_vampires(vampires);
        target_ref.set_role(game, RoleState::Vampire(Vampire::default()));
    }
    
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        let time_passed = if let Some(night_last_converted) = game.teams.vampire().night_last_converted{
            night_last_converted <= game.phase_machine.day_number - 2
        }else{
            true
        };

        actor_ref != target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.chosen_targets(game).is_empty() &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        !Team::same_team(actor_ref.role(game), target_ref.role(game)) &&
        game.teams.vampire().orderd_vampires.len() <= 4 &&
        game.teams.vampire().orderd_vampires.last() == Some(&actor_ref) &&
        time_passed
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {

    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Vampire])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        let mut out = crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref);
        out.push(ChatGroup::Vampire);
        out
    }
    fn on_phase_start(self, _game: &mut Game, _actor_ref: PlayerReference, _phase: PhaseType){
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
}