use crate::{game::{ability_input::{AvailablePlayerListSelection, AvailableSelectionKind, ControllerID, ControllerParameters}, components::{detained::Detained, insider_group::InsiderGroupID}, phase::PhaseType, player::PlayerReference, Game}, vec_set::VecSet};

use super::ControllerParametersMap;

pub struct NoAbilitySelection;

pub trait BuilderTypeState {
    type Selection;
}
impl<A: AvailableSelectionKind> BuilderTypeState for A {
    type Selection = A::Selection;
}
impl BuilderTypeState for NoAbilitySelection {
    type Selection = NoAbilitySelection;
}
pub trait IDState {}
impl IDState for () {}
impl IDState for ControllerID {}

pub struct ControllerParametersBuilder<A: BuilderTypeState = NoAbilitySelection, I: IDState = ()> {
    available: A,
    grayed_out: bool,
    reset_on_phase_start: Option<PhaseType>,
    dont_save: bool,
    default_selection: A::Selection,
    allowed_players: VecSet<PlayerReference>,
    id: I
}

impl ControllerParametersBuilder<NoAbilitySelection, ()> {
    pub fn new() -> Self {
        ControllerParametersBuilder {
            available: NoAbilitySelection,
            grayed_out: false,
            reset_on_phase_start: None,
            dont_save: false,
            default_selection: NoAbilitySelection,
            allowed_players: VecSet::new(),
            id: ()
        }
    }
}

impl<I: IDState> ControllerParametersBuilder<NoAbilitySelection, I> {
    pub fn available_selection<A: AvailableSelectionKind>(self, game: &Game, available: A) -> ControllerParametersBuilder<A, I> {
        let default_selection = available.default_selection(game);

        ControllerParametersBuilder {
            available,
            grayed_out: self.grayed_out,
            reset_on_phase_start: self.reset_on_phase_start,
            dont_save: self.dont_save,
            default_selection,
            allowed_players: self.allowed_players,
            id: self.id
        }
    }

    pub fn player_list_typical(
        self,
        game: &Game,
        actor_ref: PlayerReference,
        can_select_self: bool,
        can_select_insiders: bool,
    ) -> ControllerParametersBuilder<AvailablePlayerListSelection, I> {
        self.available_selection(game, AvailablePlayerListSelection {
            available_players: PlayerReference::all_players(game)
                .filter(|player|
                    if !player.alive(game){
                        false
                    }else if *player == actor_ref{
                        can_select_self
                    }else if InsiderGroupID::in_same_revealed_group(game, actor_ref, *player){
                        can_select_insiders
                    }else{
                        true
                    }

                )
                .collect(),
            can_choose_duplicates: false,
            max_players: Some(1)
        })
    }
}

impl<A: BuilderTypeState, I: IDState> ControllerParametersBuilder<A, I> {
    pub fn night_typical(self, game: &Game, actor_ref: PlayerReference) -> Self {
        self
            .add_grayed_out_condition(actor_ref.ability_deactivated_from_death(game) || Detained::is_detained(game, actor_ref))
            .reset_on_phase_start(PhaseType::Obituary)
            .allowed_players([actor_ref])
    }

    pub fn reset_on_phase_start(self, phase: PhaseType) -> Self {
        Self {
            reset_on_phase_start: Some(phase),
            ..self
        }
    }

    pub fn add_grayed_out_condition(self, grayed_out: bool) -> Self {
        Self {
            grayed_out: self.grayed_out || grayed_out,
            ..self
        }
    }

    pub fn dont_save(self) -> Self {
        Self {
            dont_save: true,
            ..self
        }
    }

    pub fn allowed_players(self, players: impl IntoIterator<Item=PlayerReference>) -> Self {
        Self {
            allowed_players: players.into_iter().collect(),
            ..self
        }
    }
}

impl<A: AvailableSelectionKind, I: IDState> ControllerParametersBuilder<A, I> {
    pub fn default_selection(self, default_selection: A::Selection) -> Self {
        Self {
            default_selection,
            ..self
        }
    }

    pub fn try_build(self, game: &Game) -> Option<ControllerParameters> {
        ControllerParameters::new(game, 
            self.available.into(), 
            self.grayed_out, 
            self.reset_on_phase_start, 
            self.dont_save, 
            self.default_selection.into(), 
            self.allowed_players
        )
    }
}

impl<A: BuilderTypeState> ControllerParametersBuilder<A, ()> {
    pub fn id(self, id: ControllerID) -> ControllerParametersBuilder<A, ControllerID> {
        ControllerParametersBuilder {
            available: self.available,
            grayed_out: self.grayed_out,
            reset_on_phase_start: self.reset_on_phase_start,
            dont_save: self.dont_save,
            default_selection: self.default_selection,
            allowed_players: self.allowed_players,
            id
        }
    }
}

impl<A: AvailableSelectionKind> ControllerParametersBuilder<A, ControllerID> {
    pub fn build_map(self, game: &Game) -> ControllerParametersMap {
        if let Some(single) = ControllerParameters::new(
            game,
            self.available.into(),
            self.grayed_out,
            self.reset_on_phase_start,
            self.dont_save,
            self.default_selection.into(),
            self.allowed_players
        ){
            ControllerParametersMap::new_controller(self.id, single)
        }else{
            ControllerParametersMap::default()
        }
    }
}