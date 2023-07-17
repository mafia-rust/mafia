use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Tag{
    Doused,
    Hexed,
    Necronomicon,
    ExecutionerTarget,
    Insane
}