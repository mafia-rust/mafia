use serde::Serialize;

use crate::game::win_condition::WinCondition;
use crate::game::{attack_power::DefensePower, chat::ChatMessageVariant};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl};

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Gossip{
    night_selection: <Self as RoleStateImpl>::RoleActionChoice,
}

impl RoleStateImpl for Gossip {
    type ClientRoleState = Self;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Investigative {return;}

        if let Some(visit) = actor_ref.night_visits(game).first(){
            
            let message = ChatMessageVariant::GossipResult {
                enemies: Gossip::enemies(game, visit.target)
            };
            
            actor_ref.push_night_message(game, message);
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, action_choice.player, false){
            return
        }

        self.night_selection = action_choice;
        actor_ref.set_role_state(game, self);
    }
    fn create_visits(self, _game: &Game, _actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_action_choice_to_visits(&self.night_selection, false)
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: crate::game::phase::PhaseType) {
        crate::on_phase_start_reset_night_selection!(self, game, actor_ref, phase);
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
                    !WinCondition::can_win_together(player_ref.win_condition(game), visited_player.win_condition(game))
                }
            )
    }
}