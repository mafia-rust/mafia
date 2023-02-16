use std::{ops::Add, iter::Sum};

#[repr(isize)]
#[derive(Clone)]
pub enum Verdict {
    Innocent = 1,
    Abstain = 0,
    Guilty = -1,
}