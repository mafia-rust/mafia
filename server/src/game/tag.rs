use serde::Serialize;


#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Tag{
    GodfatherBackup,
    Doused,
    WerewolfTracked,
    RevolutionaryTarget,
    MorticianTagged,
    PuppeteerMarionette,
    LoveLinked,
    ForfeitVote,
    Elector,
    PresidentialCandidate,
    President
}