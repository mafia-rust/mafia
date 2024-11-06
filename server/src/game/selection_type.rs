use crate::vec_set::VecSet;

use super::player::PlayerReference;


struct OnePlayerOptionSelectionType{
    not_allowed: VecSet<Option<PlayerReference>>
}
impl OnePlayerOptionSelectionType{
    pub fn check(&self, selection: &Option<PlayerReference>) -> bool{
        !self.not_allowed.contains(selection)
    }
}
struct OnePlayerOptionSelection{
    selection: Option<PlayerReference>
}


struct TwoPlayerOptionSelectionType{

}
struct TwoPlayerOptionSelection{
    selection: Option<(PlayerReference, PlayerReference)>
}