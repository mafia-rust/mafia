use std::iter::once;

use crate::{
    game::{
        ability_input::{AbilityID, AbilitySelection, AvailableAbilitySelection},
        phase::PhaseType, player::PlayerReference, Game
    },
    vec_map::VecMap,
    vec_set::VecSet
};

use super::player_available_abilities::PlayerAvailableAbilities;
use crate::vec_map::vec_map;

#[derive(Default)]
pub struct AllPlayersAvailableAbilities{
    players: VecMap<PlayerReference, PlayerAvailableAbilities>
}
impl AllPlayersAvailableAbilities{
    pub fn new_ability_fast(
        game: &Game,
        actor_ref: PlayerReference,
        id: AbilityID,
        available: AvailableAbilitySelection,
        default_selection: AbilitySelection,
        grayed_out: bool,
        reset_on_phase_start: Option<PhaseType>,
        dont_save: bool
    )->Self{
        Self{
            players: vec_map![(
                actor_ref, 
                PlayerAvailableAbilities::new_ability_fast(
                    game,
                    id,
                    available,
                    default_selection,
                    grayed_out,
                    reset_on_phase_start,
                    dont_save
                )
            )]
        }
    }
    pub fn new_one_player_ability_fast(
        game: &Game,
        actor_ref: PlayerReference,
        id: AbilityID,
        available_players: VecSet<PlayerReference>,
        default_selection: Option<PlayerReference>,
        grayed_out: bool,
        reset_on_phase_start: Option<PhaseType>,
        dont_save: bool
    )->Self{
        Self::new_ability_fast(
            game,
            actor_ref,
            id,
            AvailableAbilitySelection::new_one_player_option(
                available_players.into_iter().map(|p| Some(p)).chain(once(None)).collect()
            ),
            AbilitySelection::new_one_player_option(default_selection),
            grayed_out,
            reset_on_phase_start,
            dont_save
        )
    }
    pub fn combine_overwrite(&mut self, other: Self){
        for (player_ref, player_abilities) in other.players.into_iter(){
            if let Some(existing) = self.players.get_mut(&player_ref){
                existing.combine_overwrite(player_abilities);
            }else{
                self.players.insert(player_ref, player_abilities);
            }
        }
    }
    pub fn combine_overwrite_owned(self, other: Self)->Self{
        let mut out = self;
        out.combine_overwrite(other);
        out
    }
    pub fn players(&self)->&VecMap<PlayerReference, PlayerAvailableAbilities>{
        &self.players
    }
}