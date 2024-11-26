use serde::Serialize;


#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Tag{
    Enfranchised,
    GodfatherBackup,
    Doused,
    WerewolfTracked,
    RevolutionaryTarget,
    MorticianTagged,
    PuppeteerMarionette,
    LoveLinked,
    ForfeitVote,
    Spiraling,
    Disguise,
    SyndicateGun,
    Frame
}