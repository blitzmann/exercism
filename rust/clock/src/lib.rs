extern crate math;
use math::round;
use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    /// Given a number of minutes, determines the hours offset those minutes represent.
    /// Eg: 70 minutes is really 1 hour 10 minutes, so this will return 1
    /// -70 is 1 hour and 10 minutes in the past, so this would return -2 (not -1, this is due to rolling over into the previous hour (10:00 - 70 minutes = 8:50))
    fn hours_diff(minutes: i32) -> i32 {
        if minutes < 60 && minutes >= 0 {
            0
        } else {
            round::floor(minutes as f64 / 60.0, 0) as i32
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_diff = Clock::hours_diff(minutes);

        Clock {
            hours: (hours + hours_diff).rem_euclid(24),
            minutes: (minutes).rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        let hours_diff = Clock::hours_diff(minutes);

        Clock {
            hours: (self.hours + hours_diff).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
