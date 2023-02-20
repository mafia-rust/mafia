use std::{ops::Add, iter::Sum};

#[repr(isize)]
#[derive(Clone, Default)]
pub enum Verdict {
    Innocent = 1,
    #[default]
    Abstain = 0,
    Guilty = -1,
}