
use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::ChatMessageVariant;
use crate::game::event::before_role_switch::BeforeRoleSwitch;
use crate::game::grave::GraveInformation;
use crate::game::grave::GraveReference;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::tag::Tag;
use crate::game::visit::Visit;

use crate::game::Game;
use super::Role;
use super::{Priority, RoleState, RoleStateImpl};


#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mortician {
    obscured_players: Vec<PlayerReference>,
    night_selection: super::common_role::RoleActionChoiceOnePlayer
}

pub(super) const FACTION: Faction = Faction::Mafia;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

const MAX_CREMATIONS: u8 = 3;

impl RoleStateImpl for Mortician {
    type ClientRoleState = Mortician;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return}

        if self.obscured_players.len() as u8 >= MAX_CREMATIONS {return}

        match priority {
            Priority::Deception=>{
                let Some(visit) = actor_ref.night_visits(game).first() else{return};

                let target_ref = visit.target;
                
                if !self.obscured_players.contains(&target_ref){
                    self.obscured_players.push(target_ref);
                    actor_ref.set_role_state(game, RoleState::Mortician(self));
                    actor_ref.push_player_tag(game, target_ref, Tag::MorticianTagged);
                }
            },
            _ => {}
        }
    }
    fn on_role_action(mut self, game: &mut Game, actor_ref: PlayerReference, action_choice: Self::RoleActionChoice) {
        if game.current_phase().phase() != crate::game::phase::PhaseType::Night {return};
        let Some(target_ref) = action_choice.player else {
            self.night_selection = action_choice;
            actor_ref.set_role_state(game, self);
            return;
        };

        if !(
            actor_ref != target_ref &&
            !actor_ref.night_jailed(game) &&
            actor_ref.alive(game) &&
            target_ref.alive(game) &&
            (self.obscured_players.len() as u8) < MAX_CREMATIONS && 
            !self.obscured_players.contains(&target_ref)
        ){
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
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, event: BeforeRoleSwitch) {
        if event.player() == actor_ref && event.new_role().role() != Role::Mortician {
            actor_ref.remove_player_tag_on_all(game, Tag::MorticianTagged);
        }
    }
    fn on_grave_added(self, game: &mut Game, actor_ref: PlayerReference, grave_ref: GraveReference){
        if actor_ref.alive(game) && self.obscured_players.contains(&grave_ref.deref(game).player) {
            actor_ref.add_private_chat_message(game, ChatMessageVariant::PlayerRoleAndAlibi{
                player: grave_ref.deref(game).player,
                role: grave_ref.deref(game).player.role(game),
                will: grave_ref.deref(game).player.will(game).to_string(),
            });

            grave_ref.deref_mut(game).information = GraveInformation::Obscured;
        }
    }
}
