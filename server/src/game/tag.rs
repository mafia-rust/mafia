use serde::{Deserialize, Serialize};


#[derive(Deserialize, PartialOrd, Ord, Debug, Clone, PartialEq, Eq, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Tag{
    Enfranchised,
    GodfatherBackup,
    Doused,
    WerewolfTracked,
    RevolutionaryTarget,
    MorticianTagged,
    PuppeteerMarionette,
    ForfeitVote,
    Spiraling,
    Disguise,
    SyndicateGun,
    Frame
}