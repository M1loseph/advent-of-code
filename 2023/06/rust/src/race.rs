use std::{error::Error, fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub struct RaceError {
    message: String,
}

impl RaceError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for RaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl Error for RaceError {}

impl From<std::io::Error> for RaceError {
    fn from(error: std::io::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl From<ParseIntError> for RaceError {
    fn from(value: ParseIntError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

pub struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    pub fn count_all_winning_button_hold_time(&self) -> u64 {
        (1..self.time)
            .into_iter()
            .map(|hold_time| {
                let speed = hold_time;
                let time_remaining = self.time - hold_time;
                let distance = speed * time_remaining;
                distance
            })
            .filter(|distance| distance > &self.record_distance)
            .count() as u64
    }
}

pub struct RaceBuilder {
    time: Option<u64>,
    record_distance: Option<u64>,
}

impl RaceBuilder {
    pub fn new() -> Self {
        Self {
            time: None,
            record_distance: None,
        }
    }

    pub fn with_time(&mut self, time: u64) -> &mut Self {
        self.time = Some(time);
        self
    }

    pub fn with_time_str(&mut self, time: &str) -> Result<&mut Self, RaceError> {
        Ok(self.with_time(time.parse()?))
    }

    pub fn with_record_distance(&mut self, record_distance: u64) -> &mut Self {
        self.record_distance = Some(record_distance);
        self
    }

    pub fn with_record_distance_str(
        &mut self,
        record_distance: &str,
    ) -> Result<&mut Self, RaceError> {
        Ok(self.with_record_distance(record_distance.parse()?))
    }

    pub fn build(&self) -> Result<Race, RaceError> {
        let race = Race {
            time: self.time.ok_or_else(|| RaceError::new("Missing time"))?,
            record_distance: self
                .record_distance
                .ok_or_else(|| RaceError::new("Missing record_distance"))?,
        };
        Ok(race)
    }
}

mod tests {
    #[test]
    fn test_count_all_winning_button_hold_time() {
        let race = super::Race {
            time: 7,
            record_distance: 9,
        };
        assert_eq!(race.count_all_winning_button_hold_time(), 4);
    }
}
