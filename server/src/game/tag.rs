use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Tag{
    GodfatherBackup,
    Doused,
    WerewolfTracked,
    ProvocateurTarget,
    MorticianTagged,
}