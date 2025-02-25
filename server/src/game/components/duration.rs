use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// All operators are saturating
pub enum Duration {
    Temporary(u8),
    Permanent,
}

impl Duration {
    ///Returns true if the duration is not 0
    pub fn decrement(&mut self) -> bool {
        match self {
            Self::Permanent => true,
            Self::Temporary(duration) => {
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
            Self::Temporary(duration)  => *duration += 1,
        }
    }
    
    ///Returns true if the duration is not 0
    pub fn is_over(&self) -> bool {
        match self {
            Self::Permanent => false,
            Self::Temporary(duration)  => {
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
            Self::Temporary(duration) => {
                let rhs_duration = duration;
                match rhs {
                    Self::Permanent => Self::Permanent,
                    Self::Temporary(duration) => Self::Temporary(rhs_duration.saturating_add(duration)),
                }
            }
        }
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        match rhs {
            Self::Permanent => *self = Self::Permanent,
            Self::Temporary(duration) => {
                let rhs_duration = duration;
                match self {
                    Self::Permanent => return,
                    Self::Temporary (duration) => {
                        *duration = rhs_duration.saturating_add(*duration);
                    }
                }
            }
        }
    }
}

impl Add<u8> for Duration {
    type Output = Duration;
    fn add(self, rhs: u8) -> Self::Output {
        match self {
            Self::Permanent => Self::Permanent,
            Self::Temporary (duration) => Self::Temporary(rhs.saturating_add(duration))
        }
    }
}

impl AddAssign<u8> for Duration {
    fn add_assign(&mut self, rhs: u8) {
        match self {
            Self::Permanent => return,
            Self::Temporary(duration) => {
                *duration = rhs.saturating_add(*duration);
            }
        }
    }
}

impl Sub<u8> for Duration {
    type Output = Duration;
    fn sub(self, rhs: u8) -> Self::Output {
        match self {
            Self::Permanent => Self::Permanent,
            Self::Temporary (duration) => Self::Temporary(duration.saturating_sub(rhs))
        }
    }
}

impl SubAssign<u8> for Duration {
    fn sub_assign(&mut self, rhs: u8) {
        match self {
            Self::Permanent => return,
            Self::Temporary(duration) => {
                *duration = duration.saturating_sub(rhs);
            }
        }
    }
}
