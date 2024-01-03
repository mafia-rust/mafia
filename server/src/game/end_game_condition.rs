#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EndGameCondition {
    Mafia,
    Town,
    Vampire,
    Arsonist,

    None
}
impl EndGameCondition {
    pub fn can_win_together(a: EndGameCondition, b: EndGameCondition)->bool{
        a == Self::None || b == Self::None || a == b
    }
}