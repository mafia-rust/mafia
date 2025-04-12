use crate::{
    game::{
        ability_input::*,
        phase::PhaseType, player::PlayerReference, role::Role, Game
    },
    vec_set::{vec_set, VecSet}
};

use super::tags::{TagSetID, Tags};

pub struct ForfeitVote;
impl ForfeitVote{
    pub fn controller_parameters_map(game: &Game)->ControllerParametersMap {
        if !game.settings.enabled_roles.contains(&Role::Blackmailer) {
            return ControllerParametersMap::default();
        }

        ControllerParametersMap::combine(
            PlayerReference::all_players(game)
                .map(|player|
                    ControllerParametersMap::builder(game)
                        .id(ControllerID::forfeit_vote(player))
                        .available_selection(AvailableBooleanSelection)
                        .add_grayed_out_condition(!player.alive(game) || game.current_phase().phase() != PhaseType::Discussion)
                        .reset_on_phase_start(PhaseType::Obituary)
                        .allow_players(vec_set![player])
                        .build_map()
                )
        )
    }

    /// Must go before saved_controllers on phase start
    pub fn on_phase_start(game: &mut Game, phase: PhaseType){
        match phase {
            PhaseType::Nomination => {
                for player in PlayerReference::all_players(game){
                    let choose_forfiet = matches!(game.saved_controllers.get_controller_current_selection_boolean(ControllerID::forfeit_vote(player)),Some(BooleanSelection(true)));
                    if 
                        (player.night_silenced(game) || choose_forfiet) &&
                        player.alive(game)
                    {
                        Tags::add_tag(game, TagSetID::ForfeitVote, player);
                    }
                }
            },
            PhaseType::Dusk => {
                Tags::set_tagged(game, TagSetID::ForfeitVote, &VecSet::new());
            },
            _ => {}
        }
    }

    pub fn on_game_start(game: &mut Game){
        Tags::set_viewers(game, TagSetID::ForfeitVote, &PlayerReference::all_players(game).collect());
    }

    pub fn forfeit_vote(game: &Game, player: PlayerReference)->bool{
        Tags::has_tag(game, TagSetID::ForfeitVote, player)
    }
}