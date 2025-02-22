use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::components::detained::Detained;
use crate::game::grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::role::BooleanSelection;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::vec_set;

use super::{AbilitySelection, ControllerID, ControllerParametersMap, Priority, Role, RoleState, RoleStateImpl};

#[derive(PartialEq, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Martyr {
    pub state: MartyrState
}


#[derive(PartialEq, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum MartyrState {
    Won,
    StillPlaying {
        bullets: u8
    },
    LeftTown
}

impl Default for Martyr {
    fn default() -> Self {
        Self{
            state: MartyrState::StillPlaying { bullets: 3 }
        }
    }
}


pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Martyr {
    // More information is being sent than needed by the client.
    // This should be fixed later
    type ClientRoleState = Martyr;
    fn new_state(game: &Game) -> Self {
        Self{
            state: MartyrState::StillPlaying { bullets: game.num_players().div_ceil(5) }
        }
    }
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}
        let MartyrState::StillPlaying { bullets } = self.state else {return};
        if bullets == 0 {return}
        let actor_visits = actor_ref.untagged_night_visits_cloned(game);
        if let Some(visit) = actor_visits.first() {
            let target_ref = visit.target;

            self.state = MartyrState::StillPlaying { bullets: bullets.saturating_sub(1) };

            if target_ref == actor_ref {
                if target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Suicide, AttackPower::Basic, true) {
                    self.state = MartyrState::Won;
                }
            } else {
                target_ref.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Martyr), AttackPower::Basic, true);
            }
        };

        actor_ref.set_role_state(game, self);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Martyr, 0),
            super::AvailableAbilitySelection::Boolean,
            AbilitySelection::new_boolean(false),
            match self.state {
                MartyrState::StillPlaying { bullets } => bullets == 0,
                _ => true
            } ||
            actor_ref.ability_deactivated_from_death(game) || 
            Detained::is_detained(game, actor_ref) ||
            game.day_number() <= 1,
            Some(PhaseType::Obituary),
            false,
            vec_set!(actor_ref)
        )
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        let Some(AbilitySelection::Boolean {selection: BooleanSelection(true)}) = game.saved_controllers.get_controller_current_selection(ControllerID::role(actor_ref, Role::Martyr, 0)) else {return Vec::new()};
        vec![Visit::new_none(actor_ref, actor_ref, true)]
    }
    fn on_phase_start(self,  game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if phase == PhaseType::Obituary && matches!(self.state, MartyrState::StillPlaying {..}) {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MartyrFailed);
        }

        if phase == PhaseType::Obituary && actor_ref.alive(game) && matches!(self.state, MartyrState::StillPlaying { bullets: 0 }) {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }
    fn on_role_creation(self,  game: &mut Game, actor_ref: PlayerReference) {
        game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MartyrRevealed { martyr: actor_ref.index() });
        for player in PlayerReference::all_players(game){
            player.insert_role_label(game, actor_ref);
        }
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference) {
        let left_town = game.graves.iter().any(|grave| 
            grave.player == dead_player_ref &&
            if let GraveInformation::Normal { death_cause, .. } = &grave.information {
                death_cause == &GraveDeathCause::LeftTown
            } else {false}
        );

        if dead_player_ref == actor_ref && !left_town {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MartyrWon);
            
            for player in PlayerReference::all_players(game) {
                if player == actor_ref {continue}
                if !player.alive(game) {continue}
                if player.defense(game).can_block(AttackPower::ProtectionPiercing) {continue}
                player.die(game, Grave::from_player_suicide(game, player));
            }
    
            actor_ref.set_role_state(game, RoleState::Martyr(Martyr {
                state: MartyrState::Won
            }));
        }
    }
}

impl Martyr{
    pub fn won(&self)->bool{
        self.state == MartyrState::Won
    }
}
