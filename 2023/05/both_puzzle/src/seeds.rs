use std::fmt::Debug;

use crate::{almanac::AlmanacBuilder, error::AlmanacError, file_source::Strategy};

pub trait Seeds: Debug {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = u64> + 'a>;
    fn count(&self) -> u64;
}

pub trait SeedsSourceStrategy: Strategy {
    fn build_seeds(&self) -> Box<dyn Seeds>;
}

#[derive(Debug)]
pub struct SeedList {
    seeds: Vec<u64>,
}

impl Seeds for SeedList {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = u64> + 'a> {
        let iter = self.seeds.iter().cloned();
        Box::new(iter)
    }

    fn count(&self) -> u64 {
        self.seeds.len() as u64
    }
}

pub struct SeedListSourceStrategy {
    seeds: Vec<u64>,
}

impl SeedListSourceStrategy {
    pub fn new() -> SeedListSourceStrategy {
        SeedListSourceStrategy { seeds: Vec::new() }
    }
}

impl Strategy for SeedListSourceStrategy {
    fn parse_next(&mut self, line: String) -> Result<bool, AlmanacError> {
        if line == "" && self.seeds.len() > 0 {
            return Ok(true);
        }

        let (separator_index, _) = line
            .chars()
            .enumerate()
            .find(|(_, c)| *c == ':')
            .ok_or_else(|| AlmanacError {
                message: format!("No separator ':' found in seeds line: \"{}\"", line),
            })?;

        let seed_numbers = line
            .chars()
            .skip(separator_index + 1)
            .collect::<String>()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|seed_str| {
                seed_str.parse::<u64>().map_err(|err| AlmanacError {
                    message: format!("Invalid seed value {} {}", seed_str, err),
                })
            })
            .collect::<Result<Vec<u64>, AlmanacError>>()?;

        self.seeds.extend(seed_numbers);

        Ok(false)
    }

    fn add_to_builder(&self, builder: &mut AlmanacBuilder) -> Result<(), AlmanacError> {
        let seeds = Box::new(SeedList {
            seeds: self.seeds.clone(),
        });
        builder.add_seeds(seeds)
    }
}

impl SeedsSourceStrategy for SeedListSourceStrategy {
    fn build_seeds(&self) -> Box<dyn Seeds> {
        Box::new(SeedList {
            seeds: self.seeds.clone(),
        })
    }
}

#[derive(Debug)]
pub struct RangeSeeds {
    ranges: Vec<(u64, u64)>,
}

impl Seeds for RangeSeeds {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = u64> + 'a> {
        let iter = self
            .ranges
            .iter()
            .copied()
            .flat_map(|(from, size)| (from..from + size).into_iter());
        Box::new(iter)
    }

    fn count(&self) -> u64 {
        self.ranges.iter().map(|(_, size)| size).sum()
    }
}

pub struct SeedRangeSourceStrategy {
    ranges: Vec<(u64, u64)>,
}

impl SeedRangeSourceStrategy {
    pub fn new() -> SeedRangeSourceStrategy {
        SeedRangeSourceStrategy { ranges: Vec::new() }
    }
}

impl Strategy for SeedRangeSourceStrategy {
    fn parse_next(&mut self, line: String) -> Result<bool, AlmanacError> {
        if line == "" {
            return Ok(true);
        }

        if self.ranges.len() > 0 {
            return Err(AlmanacError {
                message: "Seed range source strategy can only parse one line".to_string(),
            });
        }

        let (separator_index, _) = line
            .chars()
            .enumerate()
            .find(|(_, c)| *c == ':')
            .ok_or_else(|| AlmanacError {
                message: format!("No separator ':' found in seeds line: \"{}\"", line),
            })?;

        let numbers: Vec<u64> = line
            .chars()
            .skip(separator_index + 1)
            .collect::<String>()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|str| {
                str.parse::<u64>().map_err(|err| AlmanacError {
                    message: format!("Invalid seed value {} {}", str, err),
                })
            })
            .collect::<Result<Vec<u64>, AlmanacError>>()?;

        let ranges: Vec<(u64, u64)> = numbers
            .iter()
            .copied()
            .step_by(2)
            .zip(numbers.iter().copied().skip(1).step_by(2))
            .collect();

        self.ranges = ranges;

        Ok(false)
    }

    fn add_to_builder(&self, builder: &mut AlmanacBuilder) -> Result<(), AlmanacError> {
        let seeds = Box::new(RangeSeeds {
            ranges: self.ranges.clone(),
        });
        builder.add_seeds(seeds)
    }
}
