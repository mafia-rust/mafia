use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Tag{
    GodfatherBackup,
    Doused,
    Hexed,
    Necronomicon,
    ExecutionerTarget,
    Insane
}