use rand::seq::IteratorRandom;

use crate::{game::{player::PlayerReference, Game}, vec_map::VecMap};
/// Its not an enum that is different for every role because if you swap roles but stay confused,
/// the information you get should be consistent with your previous role's info if possible.
/// the reason why it stores whether your confused instead of removing you if your not,
/// is so if you become confused after stop being confused, you have the same confusion data.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ConfusionData{
    pub red_herrings: Vec<PlayerReference>,
    pub confused: bool,
}
impl ConfusionData {
    pub fn new(game: &Game, player: PlayerReference) -> ConfusionData {
        ConfusionData {
            red_herrings: Self::generate_red_herrings(game,player),
            confused: true,
        }
    }
    pub fn generate_red_herrings(game: &Game, player: PlayerReference) -> Vec<PlayerReference> {
        let count = game.assignments.iter()
        .filter(|a|a.2.is_evil())
        .count();
        PlayerReference::all_players(game)
                .filter(|p|*p != player)
                .choose_multiple(&mut rand::rng(), count)
    }
}


#[derive(Default, Clone, Debug)]
pub struct Confused(pub VecMap<PlayerReference, ConfusionData>);

impl Game {
    fn confused(&self)->&Confused{
        &self.confused
    }
    fn confused_mut(&mut self)->&mut Confused{
        &mut self.confused
    }
}

impl Confused {
    pub fn add_player(game: &mut Game, player: PlayerReference) {
        match game.confused_mut().0.get_mut(&player) {
            Some(data) => data.confused = true,
            _=> {
                let data = ConfusionData::new(game, player);
                game.confused_mut().0.insert_unsized(player, data);
            }
        }
    }
    
    pub fn remove_player(game: &mut Game, player: PlayerReference) -> bool {
        match game.confused_mut().0.get_mut(&player) {
            Some(data) => {
                let old = data.confused.clone();
                data.confused = false;
                old
            },
            None=>false,
        }
    }

    pub fn is_confused(game: &Game, player: PlayerReference)->bool{
        Self::get_confusion_data(game, player).is_some_and(|data|data.confused)
    }

    pub fn is_red_herring(game: &Game, confused_player: PlayerReference, target: PlayerReference) -> bool{
        match game.confused().0.get(&confused_player) {
            Some(data) => data.confused && data.red_herrings.contains(&target),
            None => false,
        }
    }

    /// <strong>WARNING: JUST BECAUSE A PLAYER HAS CONFUSION DATA, IT DOESN'T MEAN THEY ARE CONFUSED</strong> <br>
    /// to check if player is confused, either check whether the data's confused flag is true
    /// or use is_confused function
    pub fn get_confusion_data(game: &Game, player: PlayerReference) -> Option<&ConfusionData>{
        game.confused().0.get(&player)
    }
}