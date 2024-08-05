use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum PlayerGroup {
    All,
    Dead,

    Mafia,
    Cult,

    Jail,
    Interview
}