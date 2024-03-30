use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::team::Team;
use crate::game::Game;
use super::{Priority, RoleStateImpl, RoleState};

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);

#[derive(Clone, Debug, Default, Serialize)]
pub struct Witch{
    currently_used_player: Option<PlayerReference> 
}

impl RoleStateImpl for Witch {
    fn defense(&self, _game: &Game, _actor_ref: PlayerReference) -> u8 {0}
    fn team(&self, _game: &Game, _actor_ref: PlayerReference) -> Option<Team> {Some(Team::Mafia)}


    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        match priority {
            Priority::Control => {

                let witch_visits = actor_ref.night_visits(game).clone();
                let Some(first_visit) = witch_visits.get(0) else {return};
                let Some(second_visit) = witch_visits.get(1) else {return};
                if !first_visit.target.alive(game) {return;}
                
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
                    first_visit.target.night_visits(game).into_iter().map(|v|v.target).collect::<Vec<PlayerReference>>();
                if let Some(target) = new_chosen_targets.first_mut(){
                    *target = second_visit.target;
                }else{
                    new_chosen_targets = vec![second_visit.target];
                }

                first_visit.target.set_night_visits(
                    game,
                    first_visit.target.convert_targets_to_visits(game, new_chosen_targets)
                );

                actor_ref.set_role_state(game, RoleState::Witch(Witch { currently_used_player: Some(first_visit.target) }));
                actor_ref.set_night_visits(game, vec![first_visit.clone()]);
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
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        ((
            actor_ref.chosen_targets(game).is_empty() &&
            !Team::same_team(game, actor_ref, target_ref)
        ) || (
            actor_ref != target_ref &&
            actor_ref.chosen_targets(game).len() == 1
        ))
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
    
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
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
            actor_ref.set_role_state(game, RoleState::Witch(Witch { currently_used_player: None }));
        }
    }
    fn on_role_creation(self, _game: &mut Game, _actor_ref: PlayerReference){
        
    }
    fn on_any_death(self, _game: &mut Game, _actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
    }
    fn on_game_ending(self, _game: &mut Game, _actor_ref: PlayerReference){
    }
}
