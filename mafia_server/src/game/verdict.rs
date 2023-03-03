use std::{ops::Add, iter::Sum};

use serde::{Serialize, Deserialize};

#[repr(isize)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Verdict {
    Innocent = 1,
    #[default]
    Abstain = 0,
    Guilty = -1,
}