use std::collections::HashSet;
use std::fs::read_to_string;

const GUARD_CURRENT_POSITION: char = '^';
const OBSTACLE: char = '#';
const PATH: char = '.';

#[derive(Clone)]
struct LaboratoryMap {
    guard_position: (usize, usize),
    guard_facing: Direction,
    obstracles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    OBSTACLE,
    PATH,
}

impl LaboratoryMap {
    fn perform_guard_step(&mut self) -> bool {
        let (x, y) = self.guard_position;
        return match self.guard_facing {
            Direction::UP => {
                if y == 0 {
                    false
                } else {
                    match self.obstracles[y - 1][x] {
                        Tile::OBSTACLE => {
                            self.guard_facing = self.guard_facing.turn_right();
                        }
                        Tile::PATH => {
                            self.guard_position = (x, y - 1);
                        }
                    }
                    true
                }
            }
            Direction::DOWN => {
                if y == self.obstracles.len() - 1 {
                    false
                } else {
                    match self.obstracles[y + 1][x] {
                        Tile::OBSTACLE => {
                            self.guard_facing = self.guard_facing.turn_right();
                        }
                        Tile::PATH => {
                            self.guard_position = (x, y + 1);
                        }
                    }
                    true
                }
            }
            Direction::LEFT => {
                if x == 0 {
                    false
                } else {
                    match self.obstracles[y][x - 1] {
                        Tile::OBSTACLE => {
                            self.guard_facing = self.guard_facing.turn_right();
                        }
                        Tile::PATH => {
                            self.guard_position = (x - 1, y);
                        }
                    }
                    true
                }
            }
            Direction::RIGHT => {
                if x == self.obstracles[y].len() - 1 {
                    false
                } else {
                    match self.obstracles[y][x + 1] {
                        Tile::OBSTACLE => {
                            self.guard_facing = self.guard_facing.turn_right();
                        }
                        Tile::PATH => {
                            self.guard_position = (x + 1, y);
                        }
                    }
                    true
                }
            }
        };
    }

    fn cound_visited_positions(&mut self) -> usize {
        let mut visited_positions = HashSet::new();

        loop {
            visited_positions.insert(self.guard_position);
            if !self.perform_guard_step() {
                break;
            }
        }
        visited_positions.len()
    }

    fn loop_exists(&mut self) -> bool {
        let mut visited_positions = HashSet::new();

        loop {
            let (x, y) = self.guard_position;
            if !visited_positions.insert((x, y, self.guard_facing)) {
                return true;
            }
            if !self.perform_guard_step() {
                return false;
            }
        }
    }

    fn count_new_obstacles_that_cause_loop(&self) -> u64 {
        let mut accumulator = 0u64;
        for y in 0..self.obstracles.len() {
            for x in 0..self.obstracles[y].len() {
                if self.obstracles[y][x] == Tile::OBSTACLE {
                    continue;
                }
                if self.guard_position == (x, y) {
                    continue;
                }
                // TODO: check performance improvement without clone
                // (resetting new field and reseting guard position and facing direction)
                let mut clone = self.clone();
                clone.obstracles[y][x] = Tile::OBSTACLE;
                if clone.loop_exists() {
                    accumulator += 1;
                }
            }
        }
        accumulator
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        }
    }
}

fn puzzle_1(laboratory_map: &LaboratoryMap) {
    let unique_visited_tiles = laboratory_map.clone().cound_visited_positions();
    println!("The guard visited {unique_visited_tiles} unique tiles");
}

fn puzzle_2(laboratory_map: &LaboratoryMap) {
    let new_obstacles_that_cause_loop = laboratory_map.count_new_obstacles_that_cause_loop();
    println!("There are {new_obstacles_that_cause_loop} possible placements for a single obstacle to cause a loop");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut guard_position: (usize, usize) = (0, 0);
    let map = file_content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut map_line = Vec::new();
            for (x, char) in line.chars().enumerate() {
                match char {
                    OBSTACLE => map_line.push(Tile::OBSTACLE),
                    PATH => map_line.push(Tile::PATH),
                    GUARD_CURRENT_POSITION => {
                        guard_position = (x, y);
                        map_line.push(Tile::PATH);
                    }
                    _ => panic!("Got char {char} that is not expected"),
                }
            }
            map_line
        })
        .collect::<Vec<Vec<Tile>>>();
    let lab_map = LaboratoryMap {
        guard_facing: Direction::UP,
        guard_position: guard_position,
        obstracles: map,
    };

    puzzle_1(&lab_map);
    // TODO: measure execution time
    puzzle_2(&lab_map);
}
