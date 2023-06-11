use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Tag{
    Doused,
    Hexxed,
    Necronomicon,
    ExecutionerTarget
}