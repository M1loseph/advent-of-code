use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Range {
    from: u64,
    to: u64,
}

impl Range {
    fn includes(&self, value: &u64) -> bool {
        *value >= self.from && *value <= self.to
    }

    fn intersect(&self, other: &Range) -> bool {
        self.from <= other.to && self.to >= other.from
    }

    fn merge(&self, other: &Range) -> Range {
        let from = self.from.min(other.from);
        let to = self.to.max(other.to);
        Range { from, to }
    }

    fn size(&self) -> u64 {
        self.to - self.from + 1
    }
}

fn parse_file() -> (Vec<Range>, Vec<u64>) {
    let file_content = read_to_string("input.txt").unwrap();
    let mut parsing_range = true;
    let mut ranges = vec![];
    let mut ids = vec![];

    for line in file_content.lines() {
        if line.is_empty() {
            parsing_range = false;
            continue;
        }
        if parsing_range {
            let (from, to) = line.split_once("-").unwrap();
            ranges.push(Range {
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
            });
        } else {
            ids.push(line.parse().unwrap());
        }
    }
    (ranges, ids)
}

fn puzzle_1(ranges: &Vec<Range>, ids: &Vec<u64>) {
    let fresh_ids = ids
        .iter()
        .filter(|id| ranges.iter().any(|range| range.includes(id)))
        .count();

    println!("[Puzzle 1] There are {fresh_ids} fresh ids")
}

fn puzzle_2(ranges: &Vec<Range>) {
    let mut resulting_ranges: Vec<Range> = vec![];
    for range in ranges {
        let mut new_resulting_ranges = vec![];
        let mut range = *range;
        for existing_range in resulting_ranges {
            if existing_range.intersect(&range) {
                range = range.merge(&existing_range);
            } else {
                new_resulting_ranges.push(existing_range);
            }
        }
        new_resulting_ranges.push(range);
        resulting_ranges = new_resulting_ranges;
    }
    let all_ranges_size: u64 = resulting_ranges.iter().map(|range| range.size()).sum();
    println!("[Puzzle 2] All ranges combined include {all_ranges_size} ingredients ids");
}

fn main() {
    let (ranges, ids) = parse_file();
    puzzle_1(&ranges, &ids);
    puzzle_2(&ranges);
}
