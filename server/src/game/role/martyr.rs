use serde::Serialize;

use crate::game::attack_power::{AttackPower, DefensePower};
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::grave::{Grave, GraveDeathCause, GraveInformation, GraveKiller};
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::visit::Visit;
use crate::game::Game;

use super::{Priority, RoleStateImpl, Role, RoleState};

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
            state: MartyrState::StillPlaying { bullets: 2 }
        }
    }
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Martyr {
    // More information is being sent than needed by the client.
    // This should be fixed later
    type ClientRoleState = Martyr;
    type RoleActionChoice = super::common_role::RoleActionChoiceOnePlayer;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return}
        let MartyrState::StillPlaying { bullets } = self.state else {return};
        if bullets == 0 {return}

        if let Some(visit) = actor_ref.night_visits(game).first() {
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

        actor_ref.set_role_state(game, RoleState::Martyr(self));
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref == target_ref &&
        !actor_ref.night_jailed(game) &&
        actor_ref.selection(game).is_empty() &&
        actor_ref.alive(game) && 
        match self.state {
            MartyrState::StillPlaying { bullets } => bullets != 0,
            _ => false
        }
    }
    fn convert_selection_to_visits(self,  game: &Game, actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        crate::game::role::common_role::convert_selection_to_visits(game, actor_ref, target_refs, true)
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
