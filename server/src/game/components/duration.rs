use std::ops::{Add, AddAssign};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Duration {
    Permanent,
    Temporary { duration: u8 },
}

impl Duration {
    ///Returns true if the duration is not 0
    pub fn decrement(&mut self) -> bool {
        match self {
            Self::Permanent => true,
            Self::Temporary { duration } => {
                if *duration == 0 {
                    return false;
                }
                *duration -= 1;
                return *duration > 0;
            }
        }
    }

    pub fn increment(&mut self) {
        match self {
            Self::Permanent => (),
            Self::Temporary { duration } => *duration += 1,
        }
    }

    pub fn is_over(&self) -> bool {
        match self {
            Self::Permanent => false,
            Self::Temporary { duration } => {
                return *duration == 0;
            }
        }
    }
}

impl Add for Duration {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        match self {
            Self::Permanent => Self::Permanent,
            Self::Temporary { duration } => {
                let d = duration;
                match rhs {
                    Self::Permanent => Self::Permanent,
                    Self::Temporary { duration } => Self::Temporary { duration: duration+ d },
                }
            }
        }
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        match rhs {
            Self::Permanent => *self = Self::Permanent,
            Self::Temporary { duration } => {
                let d = duration;
                match self {
                    Self::Permanent => return,
                    Self::Temporary { duration } => {
                        *duration += d;
                    }
                }
            }
        }
    }
}

impl AddAssign<u8> for Duration {
    fn add_assign(&mut self, rhs: u8) {
        match self {
            Self::Permanent => return,
            Self::Temporary { duration } => {
                *duration += rhs;
            }
        }
    }
}

impl Add<u8> for Duration {
    type Output = Duration;
    fn add(self, rhs: u8) -> Self::Output {
        match self {
            Self::Permanent => Self::Permanent,
            Self::Temporary { duration } =>  Self::Temporary {duration: duration + rhs}
        }
    }
}

