use serde::{Deserialize, Serialize};

use crate::{game::{ability_input::{ability_selection::AbilitySelection, AbilityInput, AvailableSelectionKind, ControllerID}, components::insider_group::InsiderGroupID, player::PlayerReference, Game}, vec_set::VecSet};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerListSelection(pub Vec<PlayerReference>);

impl PlayerListSelection {
    /// Meant for setting default selections
    /// if the option is none, it returns an empty selection
    pub fn one(player: Option<PlayerReference>) -> Self {
        player.map_or_else(Self::default, |p| Self(vec![p]))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailablePlayerListSelection {
    pub available_players: VecSet<PlayerReference>,
    pub can_choose_duplicates: bool,
    pub max_players: Option<u8>
}
impl AvailableSelectionKind for AvailablePlayerListSelection{
    type Selection = PlayerListSelection;
    fn validate_selection(&self, _game: &Game, selection: &PlayerListSelection)->bool{
        self.available_players.is_superset(&selection.0.iter().copied().collect()) && 
        (self.can_choose_duplicates || selection.0.len() == selection.0.iter().collect::<Vec<_>>().len()) &&
        self.max_players.is_none_or(|max| selection.0.len() <= max as usize)
    }
    
    fn default_selection(&self, _: &Game) -> Self::Selection {
        PlayerListSelection(Vec::new())
    }
}
impl AvailablePlayerListSelection {
    pub fn single_player_selection_typical(actor_ref: PlayerReference, can_select_self: bool, can_select_insiders: bool, game: &Game) -> Self {
        AvailablePlayerListSelection {
            available_players: PlayerReference::all_players(game)
                .filter(|player|
                    if !player.alive(game){
                        false
                    }else if *player == actor_ref{
                        can_select_self
                    }else{ 
                        can_select_insiders || !InsiderGroupID::in_same_revealed_group(game, actor_ref, *player)
                    }
                )
                .collect(),
            can_choose_duplicates: false,
            max_players: Some(1)
        }
    }
    /// Returns a AvailablePlayerListSelection, with the available players being the living players 
    /// in the same insider group as the actor_ref. If the actor ref is not in an insider group, they can pick any living player.
    pub fn insider_selection(actor_ref: PlayerReference, can_select_self: bool, can_choose_duplicates: bool, max_players: Option<u8>, game: &Game) -> Self {
        if InsiderGroupID::in_any_group(game, actor_ref) {
            Self {
                available_players: PlayerReference::all_players(game)
                    .filter(|p|
                        p.alive(game) &&
                        (can_select_self || *p != actor_ref) &&
                        InsiderGroupID::in_same_revealed_group(game, actor_ref, *p)
                    )
                    .collect(),
                can_choose_duplicates,
                max_players,
            }
        } else {
            Self {
                available_players: PlayerReference::all_players(game)
                    .filter(|p|p.alive(game))
                    .collect(),
                can_choose_duplicates,
                max_players,
            }
        }
    }
}


impl AbilityInput{
    pub fn get_player_list_selection_if_id(&self, id: ControllerID)->Option<PlayerListSelection>{
        if id != self.id() {return None};
        let AbilitySelection::PlayerList(selection) = self.selection() else {return None};
        Some(selection)
    }
}