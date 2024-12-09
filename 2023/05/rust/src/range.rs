#[derive(Debug, Clone)]
pub struct Range {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_size: u64,
}

impl Range {
    pub fn contains_source(&self, source: u64) -> bool {
        let source_range = self.source_range_start..self.source_range_start + self.range_size;
        source_range.contains(&source)
    }

    pub fn calculate_destination(&self, source: u64) -> u64 {
        self.destination_range_start + (source - self.source_range_start)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_contain_source() {
        let range = super::Range {
            destination_range_start: 0,
            source_range_start: 10,
            range_size: 10,
        };

        assert!(range.contains_source(10));
        assert!(range.contains_source(19));
        assert!(!range.contains_source(9));
        assert!(!range.contains_source(20));
    }

    #[test]
    fn should_calculate_destination() {
        let range = super::Range {
            destination_range_start: 0,
            source_range_start: 10,
            range_size: 10,
        };

        assert_eq!(range.calculate_destination(10), 0);
        assert_eq!(range.calculate_destination(19), 9);
    }
}
