use crate::map::Map;

use super::error::AlmanacError;

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<Map>,
}

impl Almanac {
    pub fn find_lowest_location_for_all_seeds(&self) -> Result<u64, AlmanacError> {
        let locations = self
            .seeds
            .iter()
            .map(|seed| self.find_location_for_seed(*seed))
            .collect::<Result<Vec<u64>, AlmanacError>>()?;

        locations.into_iter().min().ok_or_else(|| AlmanacError {
            message: "No locations found".to_string(),
        })
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
