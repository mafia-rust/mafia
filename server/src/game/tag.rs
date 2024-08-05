use serde::Serialize;


#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Tag{
    GodfatherBackup,
    Doused,
    WerewolfTracked,
    ProvocateurTarget,
    MorticianTagged,
    PuppeteerMarionette,
}