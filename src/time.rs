use std::fmt;
use std::fmt::write;
use std::ops::Add;

fn mod_remainder(k: u32, base: u32) -> (u32, u32) {
    ( k % base, k / base )
}

#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Time {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self { hours: hours % 60, minutes: minutes % 60, seconds: seconds % 24 }
    }
}

impl Add for Time {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (seconds, seconds_remainder) = mod_remainder(self.seconds + rhs.seconds, 60);
        let (minutes, minutes_remainder) = mod_remainder(self.minutes + rhs.minutes + seconds_remainder, 60);
        let (hours, _) = mod_remainder(self.hours + rhs.hours + minutes_remainder, 24);

        Self { hours, minutes, seconds }
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