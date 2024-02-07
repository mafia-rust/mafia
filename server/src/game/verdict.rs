use serde::{Serialize, Deserialize};

#[repr(isize)]
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum Verdict {
    Innocent = 1,
    #[default]
    Abstain = 0,
    Guilty = -1,
}