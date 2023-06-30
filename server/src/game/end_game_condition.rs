#[derive(Debug, PartialEq, Eq)]
pub enum EndGameCondition {
    Mafia,
    Coven,
    Town,

    Vampire,

    None
}
impl EndGameCondition {
    ///The jester can win with everyone
    ///The serial killer only wins with other serial killers
    pub fn can_win_together(a: EndGameCondition, b: EndGameCondition)->bool{
        a == Self::None || b == Self::None || a == b
    }
}