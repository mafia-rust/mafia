use super::role::Role;

#[derive(Debug, PartialEq, Eq)]
pub enum EndGameCondition {
    SingleRole,      //arsonist, werewolf, serial killer, pestilence, juggernaut
    Faction,
    None
}
impl EndGameCondition {
    ///The jester can win with everyone
    ///The serial killer only wins with other serial killers
    pub fn wins_with(a: Role, b: Role)->bool{
        if a.end_game_condition() == Self::None {return true};
        if b.end_game_condition() == Self::None {return true};
        if a.end_game_condition() != b.end_game_condition() {return false;}

        match a.end_game_condition() {
            EndGameCondition::Faction => a.faction_alignment().faction() == b.faction_alignment().faction(),
            EndGameCondition::SingleRole => a == b,
            EndGameCondition::None => true,
        }
    }
}