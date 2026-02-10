use std::fs::read_to_string;

struct Point2D {
    x: u64,
    y: u64,
}

impl Point2D {
    fn area(&self, other: &Point2D) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn puzzle1(points: &Vec<Point2D>) {
    let max_area = points
        .iter()
        .flat_map(|i| points.iter().map(|j| i.area(j)))
        .max()
        .unwrap();
    println!("[Puzzle 1] Result: {max_area}");
}

fn main() {
    let points = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point2D {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();
    puzzle1(&points);
}
