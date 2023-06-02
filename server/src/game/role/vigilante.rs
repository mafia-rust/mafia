
use crate::game::chat::night_message::NightInformation;
use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::{Grave, GraveRole, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::{Player, PlayerReference};
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleData, Role};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = true;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownKilling;
pub(super) const MAXIUMUM_COUNT: Option<u8> = None;
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;


pub(super) fn do_night_action(game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
    if actor_ref.night_jailed(game) {return}

    match priority{
        Priority::TopPriority => {
            let RoleData::Vigilante { bullets_remaining, will_suicide  } = actor_ref.role_data(game) else {unreachable!()};
            if *will_suicide {
                actor_ref.try_night_kill(game, GraveKiller::Suicide, 3);
            }
        },
        Priority::Kill => {
            
            let RoleData::Vigilante { mut bullets_remaining, mut will_suicide  } = actor_ref.role_data(game).clone() else {unreachable!()};
            if bullets_remaining < 1 || will_suicide || game.phase_machine.day_number == 1 {return;}

            if let Some(visit) = actor_ref.night_visits(game).first(){
                bullets_remaining -= 1;

                let target_ref = visit.target;
                if target_ref.night_jailed(game){
                    actor_ref.push_night_messages(game, NightInformation::TargetJailed);
                    return
                }

                let killed = target_ref.try_night_kill(game, GraveKiller::Role(Role::Vigilante), 1);

                if !killed {
                    actor_ref.push_night_messages(game,NightInformation::TargetSurvivedAttack);
                }else if target_ref.role(game).faction_alignment().faction() == Faction::Town {
                    will_suicide = true;
                }

                actor_ref.set_role_data(game, RoleData::Vigilante { bullets_remaining, will_suicide });
            }
        },
        _ => {}
    }

}
pub(super) fn can_night_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    let RoleData::Vigilante { bullets_remaining, will_suicide  } = actor_ref.role_data(game) else {unreachable!()};
    crate::game::role::common_role::can_night_target(game, actor_ref, target_ref) && *bullets_remaining > 0 && !*will_suicide && game.phase_machine.day_number != 1
}
pub(super) fn do_day_action(game: &mut Game, actor_ref: PlayerReference, target_ref: PlayerReference) {
    
}
pub(super) fn can_day_target(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
    crate::game::role::common_role::convert_targets_to_visits(game, actor_ref, target_refs, false, true)
}
pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
}
pub(super) fn get_current_recieve_chat_groups(game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
    crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
}
pub(super) fn on_phase_start(game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
}
pub(super) fn on_role_creation(game: &mut Game, actor_ref: PlayerReference){
    crate::game::role::common_role::on_role_creation(game, actor_ref);
}