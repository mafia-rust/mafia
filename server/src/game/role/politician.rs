use serde::Serialize;

use crate::game::attack_power::DefensePower;
use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::game_conclusion::GameConclusion;
use crate::game::grave::Grave;
use crate::game::modifiers::Modifiers;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;


use crate::game::tag::Tag;
use crate::game::win_condition::WinCondition;
use crate::game::Game;
use crate::vec_set;

use super::{ControllerID, ControllerParametersMap, GetClientRoleState, Role, RoleState, RoleStateImpl};


#[derive(Debug, Clone, Default)]
pub struct Politician{
    pub revealed: bool,
    state: PoliticianState,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PoliticianState{
    #[default]
    None,
    CountdownStarted,
    FinalNomination,
    Finished
}
impl PoliticianState {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
    pub fn countdown_started(&self) -> bool {
        match self {
            PoliticianState::None => false,
            PoliticianState::CountdownStarted => true,
            PoliticianState::FinalNomination => true,
            PoliticianState::Finished => false,
        }
    }
    pub fn final_nomination(&self) -> bool {
        match self {
            PoliticianState::None => false,
            PoliticianState::CountdownStarted => false,
            PoliticianState::FinalNomination => true,
            PoliticianState::Finished => false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientRoleState;


pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: DefensePower = DefensePower::Armor;

impl RoleStateImpl for Politician {
    type ClientRoleState = ClientRoleState;
    fn on_validated_ability_input_received(self, game: &mut Game, actor_ref: PlayerReference, input_player: PlayerReference, ability_input: super::AbilityInput) {
        if actor_ref != input_player {return;}
        if ability_input.id() != ControllerID::role(actor_ref, Role::Mayor, 0) {
            return;
        }
        

        game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::MayorRevealed { player_index: actor_ref.index() });

        actor_ref.set_role_state(game, Politician{
            revealed: true,
            ..self
        });
        for player in PlayerReference::all_players(game){
            player.push_player_tag(game, actor_ref, Tag::Enfranchised);
        }
        game.count_nomination_and_start_trial(
            !Modifiers::modifier_is_enabled(game, crate::game::modifiers::ModifierType::ScheduledNominations)
        );
    }
    fn controller_parameters_map(self, game: &Game, actor_ref: PlayerReference) -> ControllerParametersMap {
        ControllerParametersMap::new_controller_fast(
            game,
            ControllerID::role(actor_ref, Role::Politician, 0),
            super::AvailableAbilitySelection::Unit,
            super::AbilitySelection::new_unit(),
            actor_ref.ability_deactivated_from_death(game) ||
            self.revealed || 
            PhaseType::Night == game.current_phase().phase() ||
            PhaseType::Briefing == game.current_phase().phase(),
            None,
            true,
            vec_set![actor_ref]
        )
    }
    fn before_role_switch(self, game: &mut Game, actor_ref: PlayerReference, player: PlayerReference, _new: super::RoleState, _old: super::RoleState) {
        if actor_ref != player {return;}
        for player in PlayerReference::all_players(game){
            player.remove_player_tag(game, actor_ref, Tag::Enfranchised);
        }
    }
    fn on_phase_start(mut self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        Self::check_and_leave_town(&self, game, actor_ref);

        if self.state.countdown_started() && actor_ref.alive(game) {
            //for skipping phases
            // this litterally causes the entire server to crash
            // match phase {
            //     PhaseType::Briefing | PhaseType::Nomination | PhaseType::Testimony | 
            //     PhaseType::Judgement | PhaseType::FinalWords | PhaseType::Recess => {}

            //     PhaseType::Obituary | PhaseType::Discussion | PhaseType::Dusk | PhaseType::Night => {
            //         game.phase_machine.time_remaining = Duration::from_secs(0);
            //     }
            // }

            match phase {
                PhaseType::Nomination => {
                    self.state = PoliticianState::FinalNomination;
                    actor_ref.set_role_state(game, self);
                },
                PhaseType::Dusk => {
                    if self.state == PoliticianState::FinalNomination {
                        Politician::kill_all(game);
                    }
                },
                _ => {}
            }

        }
    }
    
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
        self.check_and_start_countdown(game, actor_ref);
    }

    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        self.check_and_start_countdown(game, actor_ref);
    }

    fn default_win_condition(self) -> crate::game::win_condition::WinCondition where RoleState: From<Self> {
        WinCondition::GameConclusionReached{win_if_any: vec![GameConclusion::Politician].into_iter().collect()}
    }
}

impl GetClientRoleState<ClientRoleState> for Politician {
    fn get_client_role_state(self, _game: &Game, _actor_ref: PlayerReference) -> ClientRoleState {
        ClientRoleState
    }
}

impl Politician {
    fn check_and_leave_town(&self, game: &mut Game, actor_ref: PlayerReference){
        if
            !self.state.countdown_started() &&
            actor_ref.alive(game) &&
            PlayerReference::all_players(game)
                .filter(|p|p.alive(game))
                .filter(|p|p.keeps_game_running(game))
                .all(|p|
                    !p.win_condition(game).is_loyalist_for(GameConclusion::Town)
                )

        {
            actor_ref.die(game, Grave::from_player_leave_town(game, actor_ref));
        }
    }

    fn check_and_start_countdown(mut self, game: &mut Game, actor_ref: PlayerReference){
        if !actor_ref.alive(game) || self.state.countdown_started() {
            return; 
        }
        
        if Self::should_start_countdown(&self, game, actor_ref) {
            Politician::start_countdown(&mut self, game);
        }

        actor_ref.set_role_state(game, self);
    }


    fn should_start_countdown(&self, game: &Game, actor_ref: PlayerReference)->bool{
        !self.state.countdown_started() &&
        actor_ref.alive(game) &&
            PlayerReference::all_players(game)
            .filter(|p|*p != actor_ref)
            .filter(|p|p.keeps_game_running(game))
            .filter(|p|p.alive(game))
            .all(|player| {
                player.win_condition(game).is_loyalist_for(GameConclusion::Town)
            })
    }

    fn start_countdown(&mut self, game: &mut Game){
        game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::PoliticianCountdownStarted);
        
        // causes the entire server to crash
        // if game.current_phase().phase() != PhaseType::Nomination {
        //     game.phase_machine.time_remaining = Duration::from_secs(0);
        // }
        self.state = PoliticianState::CountdownStarted;
    }

    fn kill_all(game: &mut Game){
        for player in PlayerReference::all_players(game){
            if player.alive(game) && !player.win_condition(game).is_loyalist_for(GameConclusion::Politician) {
                player.die(game, Grave::from_player_leave_town(game, player));
            }
        }
    }
}