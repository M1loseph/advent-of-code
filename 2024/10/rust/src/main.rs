use std::{collections::HashSet, fs::read_to_string};
use common::benchmark::{benchmark, TimeUnit};
type Height = u8;

struct Map {
    points: Vec<Vec<Height>>,
}

impl Map {
    fn new(topology: Vec<Vec<Height>>) -> Self {
        Map { points: topology }
    }

    fn depth_first_search(&self, x: usize, y: usize, visited_nines: &mut HashSet<(usize, usize)>) -> usize {
        let current_height = self.points[y][x];
        if current_height == 9 {
            visited_nines.insert((x, y));
            return 1;
        }
        let mutations = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let expected_next = current_height + 1;
        mutations.into_iter().map(|(dx, dy)| {
            let new_x = x.checked_add_signed(dx);
            let new_y = y.checked_add_signed(dy);
            match (new_x, new_y) {
                (Some(new_x), Some(new_y)) => {
                    if new_y < self.points.len()
                        && new_x < self.points[new_y].len()
                        && self.points[new_y][new_x] == expected_next
                    {
                        self.depth_first_search(new_x, new_y, visited_nines)
                    } else {
                        0
                    }
                }
                _ => {
                    0
                }
            }
        }).sum()
    }

    fn sum_of_trailhead_scores(&self) -> usize {
        (0..self.points.len())
            .into_iter()
            .flat_map(|y| {
                (0..self.points[y].len()).map(move |x| {
                    if self.points[y][x] == 0 {
                        let mut visited_nines = HashSet::new();
                        self.depth_first_search(x, y, &mut visited_nines);
                        visited_nines.len()
                    } else {
                        0
                    }
                })
            })
            .sum()
    }

    fn sum_of_trailhead_ratings(&self) -> usize {
        (0..self.points.len())
            .into_iter()
            .flat_map(|y| {
                (0..self.points[y].len()).map(move |x| {
                    if self.points[y][x] == 0 {
                        let mut visited_nines = HashSet::new();
                        self.depth_first_search(x, y, &mut visited_nines)
                    } else {
                        0
                    }
                })
            })
            .sum()
    }
}

fn puzzle_1(map: &Map) {
    let sum_of_trailheads_scores = map.sum_of_trailhead_scores();
    println!("Sum of scores of trailheads is {sum_of_trailheads_scores}");
}

fn puzzle_2(map: &Map) {
    let sum_of_trailhead_ratings = map.sum_of_trailhead_ratings();
    println!("Sum of ratings of trailheads is {sum_of_trailhead_ratings}");
}


fn main() {
    let file = read_to_string("input.txt").unwrap();
    let height_matrix = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as Height)
                .collect()
        })
        .collect();
    let map = Map::new(height_matrix);
    benchmark(|| puzzle_1(&map), TimeUnit::MICROSECONDS);
    benchmark(|| puzzle_2(&map), TimeUnit::MICROSECONDS);
}
