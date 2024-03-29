use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Tag{
    GodfatherBackup,
    Doused,
    WerewolfTracked,
    ExecutionerTarget,

    Hexed,
    Necronomicon,
    Insane
}