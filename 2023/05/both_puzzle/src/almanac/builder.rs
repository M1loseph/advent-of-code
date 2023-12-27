use super::almanac::Almanac;
use super::error::AlmanacError;
use crate::map::MapBuilder;
use crate::range::Range;

pub struct AlmanacBuilder {
    seeds: Vec<u64>,
    maps: Vec<MapBuilder>,
}

impl AlmanacBuilder {
    pub fn new() -> AlmanacBuilder {
        AlmanacBuilder {
            seeds: Vec::new(),
            maps: Vec::new(),
        }
    }
    pub fn build(self) -> Almanac {
        Almanac {
            seeds: self.seeds,
            maps: self.maps.into_iter().map(|map| map.build()).collect(),
        }
    }

    pub fn add_map_range(
        &mut self,
        source: &str,
        destination: &str,
        range: Range,
    ) -> Result<(), AlmanacError> {
        let map = self
            .maps
            .iter_mut()
            .find(|map| map.is_from_to(source, destination))
            .ok_or_else(|| AlmanacError {
                message: format!(
                    "No map found for source {} and destination {}",
                    source, destination
                ),
            })?;
        map.add_range(range);
        Ok(())
    }

    pub fn add_new_map(&mut self, map_builder: MapBuilder) -> Result<(), AlmanacError> {
        if self
            .maps
            .iter()
            .find(|map| map.is_from_to(&map_builder.source, &map_builder.destination))
            .is_some()
        {
            return Err(AlmanacError {
                message: format!(
                    "Map from {} to {} already exists",
                    map_builder.source, map_builder.destination
                ),
            });
        }
        self.maps.push(map_builder);
        Ok(())
    }

    pub fn set_seeds(&mut self, seeds: Vec<u64>) -> Result<(), AlmanacError> {
        if seeds.is_empty() {
            return Err(AlmanacError {
                message: "Provided seed list is empty".to_string(),
            });
        }
        if !self.seeds.is_empty() {
            return Err(AlmanacError {
                message: "Seeds have already been set".to_string(),
            });
        }
        self.seeds = seeds;
        Ok(())
    }
}
