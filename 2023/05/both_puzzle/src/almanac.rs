use std::time::SystemTime;

use crate::map::Map;

use super::error::AlmanacError;
use super::seeds::Seeds;

#[derive(Debug)]
pub struct Almanac {
    seeds: Box<dyn Seeds>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn find_lowest_location_for_all_seeds(&self) -> Result<u64, AlmanacError> {
        let mut measurer = TimeMeasurement::new(self.seeds.count());
        let mut min = u64::MAX;
        for seed in self.seeds.iter() {
            measurer.measure(|| {
                let location = self.find_location_for_seed(seed)?;
                if location < min {
                    min = location;
                }
                Ok(())
            })?;
        }
        Ok(min)
    }

    fn find_location_for_seed(&self, seed: u64) -> Result<u64, AlmanacError> {
        let mut key = seed;
        let mut current_map = self.find_map_by_source("seed")?;
        while current_map.destination != "location" {
            key = current_map.find_destination(key);
            current_map = self.find_map_by_source(current_map.destination.as_str())?;
        }
        Ok(current_map.find_destination(key))
    }

    fn find_map_by_source(&self, source: &str) -> Result<&Map, AlmanacError> {
        self.maps
            .iter()
            .find(|map| map.source == source)
            .ok_or_else(|| AlmanacError {
                message: format!("No map found for source {}", source),
            })
    }
}

pub struct AlmanacBuilder {
    seeds: Option<Box<dyn Seeds>>,
    maps: Vec<Map>,
}

impl AlmanacBuilder {
    pub fn new() -> AlmanacBuilder {
        AlmanacBuilder {
            seeds: Option::None,
            maps: Vec::new(),
        }
    }
    pub fn build(self) -> Result<Almanac, AlmanacError> {
        let almanac = Almanac {
            seeds: self.seeds.ok_or_else(|| AlmanacError {
                message: "Seeds have not been set".to_string(),
            })?,
            maps: self.maps,
        };
        Ok(almanac)
    }

    pub fn add_new_map(&mut self, new_map: Map) -> Result<(), AlmanacError> {
        if self
            .maps
            .iter()
            .find(|map| map.source == new_map.source && map.destination == new_map.destination)
            .is_some()
        {
            return Err(AlmanacError {
                message: format!(
                    "Map from {} to {} already exists",
                    new_map.source, new_map.destination
                ),
            });
        }
        self.maps.push(new_map);
        Ok(())
    }

    pub fn add_seeds(&mut self, seeds: Box<dyn Seeds>) -> Result<(), AlmanacError> {
        match &self.seeds {
            Some(_) => Err(AlmanacError {
                message: "Seeds have already been set".to_string(),
            }),
            None => {
                self.seeds = Some(seeds);
                Ok(())
            }
        }
    }
}

struct TimeMeasurement {
    iterations_over_second: u64,
    iterations_so_far: u64,
    total_expected_iterations: u64,
    last_time: SystemTime,
}

impl TimeMeasurement {
    fn new(expected_iterations: u64) -> TimeMeasurement {
        TimeMeasurement {
            iterations_over_second: 0,
            iterations_so_far: 0,
            total_expected_iterations: expected_iterations,
            last_time: SystemTime::now(),
        }
    }
    fn measure(
        &mut self,
        mut measured: impl FnMut() -> Result<(), AlmanacError>,
    ) -> Result<(), AlmanacError> {
        measured()?;
        self.iterations_over_second += 1;

        if SystemTime::now()
            .duration_since(self.last_time)
            .unwrap()
            .as_secs()
            > 1
        {
            self.iterations_so_far += self.iterations_over_second;
            let percent =
                self.iterations_so_far as f64 / self.total_expected_iterations as f64 * 100.0;
            println!(
                "{} iterations over 1 second, {percent:.2}% done",
                self.iterations_over_second
            );
            self.last_time = SystemTime::now();
            self.iterations_over_second = 0;
        }
        Ok(())
    }
}
