use rand::thread_rng;
use serde::Serialize;

use crate::game::chat::ChatMessageVariant;
use crate::game::grave::Grave;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;
use super::{Priority, RoleStateImpl};
use rand::prelude::SliceRandom;


#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scarecrow;

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Scarecrow {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Ward {return;}
        
        if let Some(visit) = actor_ref.night_visits(game).first(){
            let target_ref = visit.target;

            let mut blocked_players = target_ref.ward(game);
            blocked_players.shuffle(&mut thread_rng());

            let message = ChatMessageVariant::ScarecrowResult { players:
                PlayerReference::ref_vec_to_index(blocked_players.as_slice())
            };

            for player_ref in blocked_players.iter(){
                actor_ref.insert_role_label(game, *player_ref);
            }
            
            actor_ref.push_night_message(game, message);
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        if
            actor_ref.alive(game) &&
            !PlayerReference::all_players(game)
                .filter(|p|p.alive(game))
                .filter(|p|p.keeps_game_running(game))
                .any(|p|
                    p.required_resolution_states_for_win(game).is_some_and(|s1|
                        actor_ref.required_resolution_states_for_win(game).is_some_and(|s2|
                            s1.is_disjoint(&s2)
                )))

        {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }
}