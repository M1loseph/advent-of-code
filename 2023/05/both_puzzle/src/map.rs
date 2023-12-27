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
    pub source: String,
    pub destination: String,
    ranges: Vec<Range>,
}

impl MapBuilder {
    pub fn new(source: &str, destination: &str) -> MapBuilder {
        MapBuilder {
            source: source.to_string(),
            destination: destination.to_string(),
            ranges: Vec::new(),
        }
    }

    pub fn is_from_to(&self, source: &str, destination: &str) -> bool {
        self.source == source && self.destination == destination
    }

    pub fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    pub fn build(self) -> Map {
        Map {
            source: self.source,
            destination: self.destination,
            ranges: self.ranges,
        }
    }
}
