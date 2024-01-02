use crate::{almanac::AlmanacBuilder, error::AlmanacError, file_source::Strategy};

use super::range::Range;

#[derive(Debug)]
pub struct Map {
    pub source: String,
    pub destination: String,
    ranges: Vec<Range>,
}

impl Map {
    pub fn find_destination(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if range.contains_source(source) {
                return range.calculate_destination(source);
            }
        }
        source
    }
}

pub struct MapBuilder {
    pub source: Option<String>,
    pub destination: Option<String>,
    ranges: Vec<Range>,
}

impl Strategy for MapBuilder {
    fn parse_next(&mut self, line: String) -> Result<bool, AlmanacError> {
        if line == "" {
            return Ok(true);
        }
        match (self.source.is_some(), self.destination.is_some()) {
            (true, true) => {
                let range = self.parse_map_range(line)?;
                self.ranges.push(range);
                Ok(false)
            }
            (false, false) => {
                let (from, to) = self.parse_map_name(&line)?;
                self.source = Some(from.to_string());
                self.destination = Some(to.to_string());
                Ok(false)
            }
            _ => Err(AlmanacError {
                message: "".to_string(),
            }),
        }
    }

    fn add_to_builder(&self, builder: &mut AlmanacBuilder) -> Result<(), AlmanacError> {
        builder.add_new_map(Map {
            source: self.source.clone().ok_or_else(|| AlmanacError {
                message: "Unable to add map, source is missing".to_string(),
            })?,
            destination: self.destination.clone().ok_or_else(|| AlmanacError {
                message: "Unable to add map, destination is missing".to_string(),
            })?,
            ranges: self.ranges.clone(),
        })
    }
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        MapBuilder {
            source: Option::None,
            destination: Option::None,
            ranges: Vec::new(),
        }
    }

    fn parse_map_range(&self, line: String) -> Result<Range, AlmanacError> {
        let elements: Vec<&str> = line.splitn(3, " ").collect();
        if elements.len() != 3 {
            return Err(AlmanacError {
                message: format!("Invalid map range line, it should contain three numbers separated with space: \"{}\"", line),
            });
        }
        Ok(Range {
            destination_range_start: elements[0].parse::<u64>()?,
            source_range_start: elements[1].parse::<u64>()?,
            range_size: elements[2].parse::<u64>()?,
        })
    }

    fn parse_map_name<'a>(&self, line: &'a String) -> Result<(&'a str, &'a str), AlmanacError> {
        let map_name = line.strip_suffix(" map:").ok_or_else(|| AlmanacError {
            message: format!(
                "Invalid map name: \"{}\", should have \" map:\" at the end",
                line
            ),
        })?;
        map_name.split_once("-to-").ok_or_else(|| AlmanacError {
            message: format!(
                "Invalid map name: \"{}\", should have '-to-' in the middle",
                map_name
            ),
        })
    }
}
