use serde::Serialize;

use crate::game::attack_power::AttackPower;
use crate::game::chat::ChatMessageVariant;
use crate::game::components::confused::Confused;
use crate::game::components::detained::Detained;
use crate::game::attack_power::DefensePower;
use crate::game::game_conclusion::GameConclusion;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;

use crate::game::visit::Visit;

use crate::game::Game;
use crate::vec_set::{vec_set, VecSet};
use super::{
    AbilitySelection, AvailableAbilitySelection, ControllerID, ControllerParametersMap,
    PlayerListSelection, Priority, Role, RoleStateImpl
};

#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Marksman {
    state: MarksmanState
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub(self) enum MarksmanState{
    #[default]
    NotLoaded,
    Loaded,
    ShotTownie
}


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::None;

impl RoleStateImpl for Marksman {
    type ClientRoleState = Marksman;
    fn do_night_action(mut self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::Kill {return};

        let visiting_players: Vec<_> = actor_ref
            .untagged_night_visits_cloned(game)
            .into_iter()
            .flat_map(|p|p.target.all_night_visitors_cloned(game))
            .collect();

        let Some(PlayerListSelection(marks)) = 
            game.saved_controllers.get_controller_current_selection_player_list(
                ControllerID::role(actor_ref, Role::Marksman, 0)
            ) else 
        {
            return;
        };
        if Confused::is_confused(game, actor_ref) {
            if !marks.is_empty() && !visiting_players.is_empty(){
                actor_ref.push_night_message(game,ChatMessageVariant::SomeoneSurvivedYourAttack);
            }
            return;
        }
        for mark in marks {
            if !visiting_players.contains(&mark) {continue};
            
            let killed = mark.try_night_kill_single_attacker(actor_ref, game, GraveKiller::Role(Role::Marksman), AttackPower::Basic, false);

            if killed && mark.win_condition(game).is_loyalist_for(GameConclusion::Town) {
                self.state = MarksmanState::ShotTownie;
            }
        }
        
        actor_ref.set_role_state(game, self);
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> super::ControllerParametersMap {
        
        let gray_out_mark = 
            actor_ref.ability_deactivated_from_death(game) || 
            Detained::is_detained(game, actor_ref) ||
            self.state != MarksmanState::Loaded;

        let available_mark_players = PlayerReference::all_players(game)
            .into_iter()
            .filter(|p|
                p.alive(game) && 
                *p != actor_ref
            )
            .collect::<VecSet<_>>();
        
        let mark_controller_param = ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Marksman, 0),
            AvailableAbilitySelection::new_player_list(
                available_mark_players,
                false,
                Some(3)
            ),
            AbilitySelection::new_player_list(vec![]),
            gray_out_mark,
            Some(PhaseType::Obituary),
            false,
            vec_set!(actor_ref)
        );


        let marked_players = 
            game.saved_controllers.get_controller_current_selection_player_list(
                ControllerID::role(actor_ref, Role::Marksman, 0)
            );


        let gray_out_camp = 
            actor_ref.ability_deactivated_from_death(game) || 
            Detained::is_detained(game, actor_ref) ||
            self.state != MarksmanState::Loaded ||
            if let Some(marked_players) = marked_players {
                marked_players.0.len() == 0
            }else{
                true
            };

            let available_camp_players = PlayerReference::all_players(game)
            .into_iter()
            .filter(|p|
                p.alive(game) && 
                *p != actor_ref
            )
            .collect::<VecSet<_>>();
        
        let camp_controller_param = ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Marksman, 1),
            AvailableAbilitySelection::new_player_list(
                available_camp_players,
                false,
                Some(3)
            ),
            AbilitySelection::new_player_list(vec![]),
            gray_out_camp,
            Some(PhaseType::Obituary),
            false,
            vec_set!(actor_ref)
        );

        mark_controller_param.combine_overwrite_owned(camp_controller_param)
    }
    fn convert_selection_to_visits(self, game: &Game, actor_ref: PlayerReference) -> Vec<Visit> {
        crate::game::role::common_role::convert_controller_selection_to_visits(
            game,
            actor_ref,
            ControllerID::role(actor_ref, Role::Marksman, 1),
            false
        )
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType) {
        if 
            matches!(phase, PhaseType::Obituary) && 
            matches!(self.state, MarksmanState::NotLoaded)
        {
            actor_ref.set_role_state(game, Marksman{state: MarksmanState::Loaded})
        }
    }
}