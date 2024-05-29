use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::GraveReference;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleState, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Necromancer { 
    used_bodies: Vec<PlayerReference>, 
    currently_used_player: Option<PlayerReference> 
}
impl RoleStateImpl for Necromancer {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Control => {

                let retributionist_visits = actor_ref.night_visits(game).clone();
                let Some(first_visit) = retributionist_visits.get(0) else {return};
                let Some(second_visit) = retributionist_visits.get(1) else {return};
                if first_visit.target.alive(game) {return;}
                
                first_visit.target.push_night_message(game,
                    ChatMessageVariant::YouWerePossessed { immune: first_visit.target.control_immune(game) }
                );
                if first_visit.target.control_immune(game) {
                    actor_ref.push_night_message(game,
                        ChatMessageVariant::TargetIsPossessionImmune
                    );
                    return;
                }

                let mut new_chosen_targets = 
                    first_visit.target.night_visits(game).iter().map(|v|v.target).collect::<Vec<PlayerReference>>();
                if let Some(target) = new_chosen_targets.first_mut(){
                    *target = second_visit.target;
                }else{
                    new_chosen_targets = vec![second_visit.target];
                }

                first_visit.target.set_night_visits(
                    game,
                    first_visit.target.convert_selection_to_visits(game, new_chosen_targets)
                );

                let mut used_bodies = self.used_bodies;
                used_bodies.push(first_visit.target);
                actor_ref.set_role_state(game, RoleState::Necromancer(Necromancer { used_bodies, currently_used_player: Some(first_visit.target) }));
                actor_ref.set_night_visits(game, vec![first_visit.clone()]);
            },
            Priority::Investigative => {
                if let Some(currently_used_player) = self.currently_used_player {
                    actor_ref.push_night_message(game,
                        ChatMessageVariant::PossessionTargetsRole { role: currently_used_player.role(game) }
                    );
                }
            },
            Priority::StealMessages => {
                if let Some(currently_used_player) = self.currently_used_player {
                    for message in currently_used_player.night_messages(game).clone() {
                        actor_ref.push_night_message(game,
                            ChatMessageVariant::TargetsMessage { message: Box::new(message.clone()) }
                        );
                    }
                }
            },
            _ => {}
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        ((
            actor_ref.selection(game).is_empty() &&
            !target_ref.alive(game) &&
            !self.used_bodies.iter().any(|p| *p == target_ref)
        ) || (
            actor_ref != target_ref &&
            actor_ref.selection(game).len() == 1 &&
            target_ref.alive(game)
        ))
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
        
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_selection_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{target: target_refs[0], attack: false}, 
                Visit{target: target_refs[1], attack: false},
            ]
        }else{
            Vec::new()
        }
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![ChatGroup::Mafia])
    }
    fn get_current_receive_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_receive_chat_groups(game, actor_ref)
    }
    fn get_won_game(self, game: &Game, actor_ref: PlayerReference) -> bool {
        crate::game::role::common_role::get_won_game(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, RoleState::Necromancer(Necromancer { used_bodies: self.used_bodies, currently_used_player: None }));
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
        
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_grave_added(self, _game: &mut Game, _actor_ref: PlayerReference, _grave_ref: GraveReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
