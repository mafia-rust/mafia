use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::cult::{Cult, CultAbility};
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Spy{
    night_selection: <Self as RoleStateImpl>::RoleActionChoice,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum SpyBug{
    Silenced, 
    Roleblocked, Wardblocked,
    Protected, 
    Transported, Possessed
}

pub(super) const FACTION: Faction = Faction::Town;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Spy {
    type ClientRoleState = Spy;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        
        if !actor_ref.alive(game) {return;}

        match priority {
            Priority::Investigative => {
                if actor_ref.night_blocked(game) {return;}

                let mut mafia_visits = vec![];
                for other_player in PlayerReference::all_players(game){
                    if other_player.role(game).faction() == Faction::Mafia{
                        mafia_visits.append(&mut other_player.night_visits(game).iter().map(|v|v.target.index()).collect());
                    }
                }
                mafia_visits.shuffle(&mut rand::thread_rng());
                
                actor_ref.push_night_message(game, ChatMessageVariant::SpyMafiaVisit { players: mafia_visits });               
            },
            Priority::SpyBug => {
                let Some(visit) = actor_ref.night_visits(game).first()else{return};

                for message in visit.target.night_messages(game).clone(){
                    if let Some(message) = match message{
                        ChatMessageVariant::Silenced => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Silenced }),
                        ChatMessageVariant::RoleBlocked { immune: _ } => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Roleblocked }),
                        ChatMessageVariant::YouWereProtected => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Protected }),
                        ChatMessageVariant::Transported => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Transported }),
                        ChatMessageVariant::YouWerePossessed { immune: _ } => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Possessed }),
                        ChatMessageVariant::Wardblocked => Some(ChatMessageVariant::SpyBug { bug: SpyBug::Wardblocked }),
                        _ => None
                    }{
                        actor_ref.push_night_message(game, message);
                    }
                };
            },
            Priority::FinalPriority => {
                if actor_ref.night_blocked(game) {return;}

                let count = PlayerReference::all_players(game).filter(|p|
                    p.role(game).faction() == Faction::Cult && p.alive(game)
                ).count() as u8;

                if count > 0 {
                    match Cult::next_ability(game) {
                        CultAbility::Convert => {
                            actor_ref.push_night_message(game, ChatMessageVariant::CultConvertsNext);
                        }
                        CultAbility::Kill => {
                            actor_ref.push_night_message(game, ChatMessageVariant::CultKillsNext);
                        }
                    }

                    actor_ref.push_night_message(game, ChatMessageVariant::SpyCultistCount { count });
                }
            }
            _=>{}
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if !crate::game::role::common_role::default_action_choice_one_player_is_valid(game, actor_ref, &action_choice, false){
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