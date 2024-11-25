use serde::Serialize;

use crate::game::components::confused::Confused;
use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Philosopher;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Philosopher {
    type ClientRoleState = Philosopher;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        let Some(first_visit) = actor_visits.get(0) else {return;};
        let Some(second_visit) = actor_visits.get(1) else {return;};

        let enemies = if Confused::is_confused(game, actor_ref) {
            false
        } else {
            Philosopher::players_are_enemies(game, first_visit.target, second_visit.target)
        };

        let message = ChatMessageVariant::SeerResult{ enemies };
        
        actor_ref.push_night_message(game, message);
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        !crate::game::components::detained::Detained::is_detained(game, actor_ref) &&
        actor_ref.alive(game) &&
        target_ref.alive(game) &&
        (
            actor_ref.selection(game).is_empty() || 
            actor_ref.selection(game).len() == 1 && *actor_ref.selection(game).get(0).unwrap() != target_ref
        )
    }
    fn convert_selection_to_visits(self, _game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit::new_none(actor_ref, target_refs[0], false),
                Visit::new_none(actor_ref, target_refs[1], false)
            ]
        } else {
            Vec::new()
        }
    }
}
impl Philosopher{
    pub fn players_are_enemies(game: &Game, a: PlayerReference, b: PlayerReference) -> bool {
        if a.has_suspicious_aura(game) || b.has_suspicious_aura(game){
            true
        }else if a.has_innocent_aura(game) || b.has_innocent_aura(game){
            false
        }else{
            !WinCondition::can_win_together(a.win_condition(game), b.win_condition(game))
        }
    }
}