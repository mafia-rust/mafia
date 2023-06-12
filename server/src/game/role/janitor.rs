
use serde::Serialize;

use crate::game::chat::night_message::NightInformation;
use crate::game::chat::ChatGroup;
use crate::game::grave::GraveRole;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::FactionAlignment;
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleState, RoleStateImpl};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = true;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::MafiaDeception;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = Some(Team::Faction);

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Janitor {
    cleans_remaining: u8,
    cleaned_ref: Option<PlayerReference>
}

impl Default for Janitor {
    fn default() -> Self {
        Janitor {
            cleans_remaining: 3,
            cleaned_ref: None
        }
    }
}

impl RoleStateImpl for Janitor {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return}

        if self.cleans_remaining == 0 {return}

        match priority {
            Priority::Deception=>{
                let Some(visit) = actor_ref.night_visits(game).first() else{return};

                let target_ref = visit.target;
                if target_ref.night_jailed(game) {
                    actor_ref.push_night_message(game, NightInformation::TargetJailed);
                }else{
                    target_ref.set_night_grave_role(game, GraveRole::Cleaned);
                    target_ref.set_night_grave_will(game, "".to_owned());
                    actor_ref.set_role_state(game, RoleState::Janitor(Janitor { cleans_remaining: self.cleans_remaining - 1, cleaned_ref: Some(target_ref) }));
                }
            },
            Priority::Investigative=>{
                if let Some(cleaned_ref) = self.cleaned_ref {
                    if cleaned_ref.night_died(game) {
                        actor_ref.push_night_message(game, NightInformation::PlayerRoleAndWill{
                            role: cleaned_ref.role(game),
                            will: cleaned_ref.will(game).to_string(),
                        });
                    }
                }
            },
            _ => {}
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_target(game, actor_ref, target_ref) && self.cleans_remaining > 0
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
        
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, false)
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        actor_ref.set_role_state(game, RoleState::Janitor(Janitor { cleans_remaining: self.cleans_remaining, cleaned_ref: None }));
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
}
