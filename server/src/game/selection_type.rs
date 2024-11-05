use super::player::PlayerReference;


struct OnePlayerOptionSelectionType{
    
}
struct OnePlayerOptionSelection{
    selection: Option<PlayerReference>
}


struct TwoPlayerOptionSelectionType{

}
struct TwoPlayerOptionSelection{
    selection: Option<(PlayerReference, PlayerReference)>
}