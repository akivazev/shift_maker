use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Sub};
use crate::util::{mod_remainder, div_rem, mod_subtract};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Time {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Time {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self { hours: hours % 24, minutes: minutes % 60, seconds: seconds % 60 }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seconds_round = if self.seconds >= 30 { 1 } else { 0 };
        let (minutes, minutes_remainder) = mod_remainder(self.minutes + seconds_round, 60);
        let (hours, _) = mod_remainder(self.hours + minutes_remainder, 24);

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Duration {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Duration {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self { hours, minutes, seconds }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seconds_round = if self.seconds >= 30 { 1 } else { 0 };
        let (minutes, minutes_remainder) = mod_remainder(self.minutes + seconds_round, 60);
        let hours = self.hours + minutes_remainder;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl Add<Time> for Time {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (seconds, seconds_remainder) = mod_remainder(self.seconds + rhs.seconds, 60);
        let (minutes, minutes_remainder) = mod_remainder(self.minutes + rhs.minutes + seconds_remainder, 60);
        let (hours, _) = mod_remainder(self.hours + rhs.hours + minutes_remainder, 24);

        Self { hours, minutes, seconds }
    }
}

impl Add<Duration> for Time {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self {
        let (seconds, seconds_remainder) = mod_remainder(self.seconds + rhs.seconds, 60);
        let (minutes, minutes_remainder) = mod_remainder(self.minutes + rhs.minutes + seconds_remainder, 60);
        let (hours, _) = mod_remainder(self.hours + rhs.hours + minutes_remainder, 24);

        Self { hours, minutes, seconds }
    }
}

impl Sub<Time> for Time {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Duration {
        if self == rhs {
            Duration::new(0, 0, 0)
        } else {
            let (seconds, minutes_carry) = mod_subtract(self.seconds, rhs.seconds, 60);
            let (minutes, hours_carry) = mod_subtract(self.minutes, rhs.minutes + minutes_carry, 60);
            let (hours, _) = mod_subtract(self.hours, rhs.hours + hours_carry, 24);

            Duration{ hours, minutes, seconds }
        }
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hours.cmp(&other.hours)
            .then(self.minutes.cmp(&other.minutes))
            .then(self.seconds.cmp(&other.seconds))
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Duration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (seconds, seconds_remainder) = mod_remainder(self.seconds + rhs.seconds, 60);
        let (minutes, minutes_remainder) = mod_remainder(self.minutes + rhs.minutes + seconds_remainder, 60);

        Self {
            hours: self.hours + rhs.hours + minutes_remainder,
            minutes,
            seconds
        }
    }
}

impl Div<u32> for Duration {
    type Output = Self;
    fn div(self, rhs: u32) -> Self {
        let (hours, hours_rem) = div_rem(self.hours, rhs);
        let (minutes, minutes_rem) = div_rem(self.minutes + 60*hours_rem, rhs);
        let seconds = (self.seconds + 60*minutes_rem) / rhs;

        Self { hours, minutes, seconds }
    }
}

impl Div<Duration> for Duration {
    type Output = u32;
    fn div(self, rhs: Duration) -> u32 {
        let self_seconds = self.seconds + 60*(self.minutes + 60*self.hours);
        let rhs_seconds = rhs.seconds + 60*(rhs.minutes + 60*rhs.hours);
        self_seconds / rhs_seconds
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hours.cmp(&other.hours)
            .then(self.minutes.cmp(&other.minutes))
            .then(self.seconds.cmp(&other.seconds))
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}