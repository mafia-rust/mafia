use serde::Serialize;

use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::resolution_state::ResolutionState;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Gossip;

impl RoleStateImpl for Gossip {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            
            let message = ChatMessageVariant::GossipResult {
                enemies: Gossip::enemies(game, visit.target)
            };
            
            actor_ref.push_night_message(game, message);
        }
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        crate::game::role::common_role::can_night_select(game, actor_ref, target_ref)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, false)
    }
}

impl Gossip {
    pub fn enemies(game: &Game, player_ref: PlayerReference) -> bool {
        match player_ref.night_appeared_visits(game) {
            Some(x) => x,
            None => player_ref.night_visits(game),
        }
            .iter()
            .map(|v|v.target.clone())
            .any(|visited_player|
                if visited_player.has_suspicious_aura(game){
                    true
                }else if visited_player.has_innocent_aura(game){
                    false
                }else{
                    !ResolutionState::can_win_together(game, player_ref, visited_player)
                }
            )
    }
}