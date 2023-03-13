use crate::game::chat::night_message::NightInformation;
use crate::game::chat::{ChatGroup, ChatMessage};
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::{Player, PlayerIndex, self};
use crate::game::role_list::FactionAlignment;
use crate::game::victory_group::VictoryGroup;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleData, Role};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = false;
pub(super) const WITCHABLE: bool = false;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownKilling;
pub(super) const MAXIUMUM_COUNT: Option<u8> = Some(1);
pub(super) const VICTORY_GROUP: VictoryGroup = VictoryGroup::Faction;

pub(super) fn do_night_action(actor_index: PlayerIndex, priority: Priority, game: &mut Game) {
    match priority {
        1 => {
            let RoleData::Veteran { alerts_remaining, alerting_tonight } = game.get_unchecked_mut_player(actor_index).role_data else {unreachable!()};
            if alerts_remaining > 0 {
                if let Some(visit) = game.get_unchecked_player(actor_index).night_variables.visits.get(0){
                    if visit.target == actor_index{
                        game.get_unchecked_mut_player(actor_index).night_variables.increase_defense_to(1);
                        game.get_unchecked_mut_player(actor_index).role_data = RoleData::Veteran { alerts_remaining: alerts_remaining-1, alerting_tonight: true };
                    }
                }
            }
        }
        9 => {
            let RoleData::Veteran { alerts_remaining, alerting_tonight } = game.get_unchecked_player(actor_index).role_data else {unreachable!()};
            
            if !alerting_tonight {return}

            for player_index in 0..game.players.len(){
                let player_index = (player_index as PlayerIndex).clone();
                for visit_index in 0..game.get_unchecked_player(player_index).night_variables.visits.len(){
                    let visit = game.get_unchecked_player(player_index).night_variables.visits.get(visit_index).unwrap();

                    if !visit.target==actor_index || visit.astral {continue}

                    game.get_unchecked_mut_player(player_index).add_chat_message(ChatMessage::NightInformation{ 
                        night_information: NightInformation::VeteranAttackedYou 
                    });

                    //Kill
                    let killed = game.try_night_kill(player_index, GraveKiller::Role(Role::Veteran), 2);
                    
                    let actor = game.get_unchecked_mut_player(actor_index);
                    actor.add_chat_message(ChatMessage::NightInformation{ 
                        night_information: NightInformation::VeteranAttackedVisitor 
                    });

                    if !killed {
                        actor.add_chat_message(ChatMessage::NightInformation{ 
                            night_information: NightInformation::TargetSurvivedAttack 
                        });
                    }
                }
            }
        }
        _=>{}
    }
}
pub(super) fn can_night_target(actor_index: PlayerIndex, target_index: PlayerIndex, game: &Game) -> bool {
    if let RoleData::Veteran { alerts_remaining, alerting_tonight } = game.get_unchecked_player(actor_index).role_data{
        actor_index == target_index &&
        alerts_remaining > 0 &&
        game.get_unchecked_player(actor_index).night_variables.chosen_targets.len() < 1 &&
        game.get_unchecked_player(actor_index).alive
    }else{
        unreachable!()
    }
}
pub(super) fn do_day_action(actor: PlayerIndex, game: &mut Game) {

}
pub(super) fn can_day_target(actor: PlayerIndex, target: PlayerIndex, game: &Game) -> bool {
    false
}
pub(super) fn convert_targets_to_visits(actor: PlayerIndex, targets: Vec<PlayerIndex>, game: &Game) -> Vec<Visit> {
    if targets.len() > 0{
        vec![Visit{ target: targets[0], astral: true, attack: false }]
    }else{
        Vec::new()
    }
}
pub(super) fn get_current_chat_groups(actor_index: PlayerIndex, game: &Game) -> Vec<ChatGroup> {
    if !game.get_unchecked_player(actor_index).alive{
        return vec![ChatGroup::Dead];
    }

    match game.phase_machine.current_state {
        crate::game::phase::PhaseType::Morning => vec![],
        crate::game::phase::PhaseType::Discussion => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Voting => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Testimony => {if game.player_on_trial == Some(actor_index) {vec![ChatGroup::All]} else {vec![]}},
        crate::game::phase::PhaseType::Judgement => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Evening => vec![ChatGroup::All],
        crate::game::phase::PhaseType::Night => vec![],
    }
}
pub fn on_phase_start(actor_index: PlayerIndex, phase: PhaseType, game: &mut Game){
    let actor = game.get_unchecked_mut_player(actor_index);
    if let RoleData::Veteran { alerts_remaining, alerting_tonight } = &mut actor.role_data {
        *alerting_tonight = false;
    }else{unreachable!()}
}