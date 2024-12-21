use common::benchmark::{benchmark, TimeUnit};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::{HashMap, HashSet}, fs::read_to_string};

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn unvisited_neighbours(
        &self,
        map: &Vec<Vec<Tile>>,
        result: &Vec<Vec<Option<u64>>>,
    ) -> Vec<Point> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        directions
            .into_iter()
            .filter_map(|(dx, dy)| {
                let nx = self.x.checked_add_signed(dx)?;
                let ny = self.y.checked_add_signed(dy)?;
                if ny >= map.len() || nx > map[ny].len() {
                    return None;
                }
                if map[ny][nx] == Tile::Wall {
                    return None;
                }
                if result[ny][nx].is_some() {
                    return None;
                }
                Some(Point { x: nx, y: ny })
            })
            .collect()
    }
}

impl Tile {
    fn empty() -> Self {
        Tile::Empty
    }
    fn wall() -> Self {
        Tile::Wall
    }
}

fn dijkstra(map: &Vec<Vec<Tile>>, start: (usize, usize)) -> Vec<Vec<Option<u64>>> {
    let mut queue: PriorityQueue<Point, Reverse<u64>> = PriorityQueue::new();
    let mut result = vec![vec![None; map[0].len()]; map.len()];
    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::Empty { .. } => {
                    let distance = if (x, y) == start { 0 } else { u64::MAX };
                    queue.push(Point { x, y }, Reverse(distance));
                }
                _ => {}
            }
        }
    }

    while let Some((next_point, distance)) = queue.pop() {
        let distance = distance.0;
        let (x, y) = (next_point.x, next_point.y);
        let neigbours = next_point.unvisited_neighbours(&map, &result);
        for neighour in neigbours {
            let neighbour_distance = distance + 1;
            queue.push(neighour, Reverse(neighbour_distance));
        }

        result[y][x] = Some(distance);
    }
    result
}

fn cheat_algorithm(
    map: &Vec<Vec<Tile>>,
    distances: &Vec<Vec<Option<u64>>>,
    cheat_length: u64,
) -> HashMap<u64, u64> {
    let find_empty_cheat_desinations = |reference_point: Point| -> HashSet<Point> {
        let mut destinations = HashSet::new();
        for dy in 0..=cheat_length {
            for dx in 0..=cheat_length - dy {
                if dx + dy == 0 {
                    continue;
                }
                let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                for (mx, my) in directions {
                    let nx = reference_point.x.checked_add_signed(dx as isize * mx);
                    let ny = reference_point.y.checked_add_signed(dy as isize * my);
                    if ny.is_none() || nx.is_none() {
                        continue;
                    }
                    let nx = nx.unwrap();
                    let ny = ny.unwrap();
                    if ny >= map.len() || nx >= map[ny].len() {
                        continue;
                    }
                    match map[ny][nx] {
                        Tile::Empty => {
                            // TODO: there are duplicates on x=0 and y=0
                            destinations.insert(Point { x: nx, y: ny });
                        }
                        Tile::Wall => {}
                    }
                }
            }
        }
        destinations
    };

    let mut cheats = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                Tile::Empty => {
                    let empty_neigbours = find_empty_cheat_desinations(Point { x, y });
                    for neighbour in empty_neigbours {
                        let (nx, ny) = (neighbour.x, neighbour.y);
                        let distance = (x.abs_diff(nx) + y.abs_diff(ny)) as u64;
                        assert_ne!(distance, 0);
                        if distances[y][x].unwrap() + distance < distances[ny][nx].unwrap() {
                            let diff = distances[ny][nx].unwrap() - distances[y][x].unwrap() - distance;
                            cheats
                                .entry(diff)
                                .and_modify(|entry| *entry += 1)
                                .or_insert(1);
                        }
                    }
                }
                Tile::Wall => {}
            }
        }
    }
    cheats
}

fn one_wall_cheat_algorithm(
    map: &Vec<Vec<Tile>>,
    distances: &Vec<Vec<Option<u64>>>,
) -> HashMap<u64, u64> {
    let mut cheats = HashMap::new();
    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            match map[y][x] {
                Tile::Empty => {}
                Tile::Wall => {
                    if map[y - 1][x] == Tile::Empty && map[y + 1][x] == Tile::Empty {
                        let diff = distances[y - 1][x]
                            .unwrap()
                            .abs_diff(distances[y + 1][x].unwrap());
                        if diff > 2 {
                            cheats
                                .entry(diff - 2)
                                .and_modify(|entry| {
                                    *entry += 1;
                                })
                                .or_insert(1);
                        }
                    }
                    if map[y][x - 1] == Tile::Empty && map[y][x + 1] == Tile::Empty {
                        let diff = distances[y][x - 1]
                            .unwrap()
                            .abs_diff(distances[y][x + 1].unwrap());
                        if diff > 2 {
                            cheats
                                .entry(diff - 2)
                                .and_modify(|entry| {
                                    *entry += 1;
                                })
                                .or_insert(1);
                        }
                    }
                }
            }
        }
    }
    cheats
}

fn puzzle_1(map: &Vec<Vec<Tile>>, start: (usize, usize), end: (usize, usize)) {
    let _ = end;
    let minimum_cheat_save = 100;
    let distances = dijkstra(map, start);
    let cheats = one_wall_cheat_algorithm(&map, &distances);
    let total_cheats: u64 = cheats
        .iter()
        .filter(|(k, _)| **k >= minimum_cheat_save)
        .map(|(_, v)| *v)
        .sum();
    println!(
        "There are {total_cheats} cheats that save you at least {minimum_cheat_save} picoseconds"
    );
}

fn puzzle_2(map: &Vec<Vec<Tile>>, start: (usize, usize), end: (usize, usize)) {
    let _ = end;
    let minimum_cheat_save = 100;
    let distances = dijkstra(map, start);
    let cheats = cheat_algorithm(&map, &distances, 20);
    let total_cheats: u64 = cheats
        .iter()
        .filter(|(k, _)| **k >= minimum_cheat_save)
        .map(|(_, v)| *v)
        .sum();
    println!(
        "There are {total_cheats} cheats that save you at least {minimum_cheat_save} picoseconds"
    );
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map: Vec<Vec<Tile>> = file_content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '#' => Tile::wall(),
                    '.' => Tile::empty(),
                    'S' => {
                        start = (x, y);
                        Tile::empty()
                    }
                    'E' => {
                        end = (x, y);
                        Tile::empty()
                    }
                    _ => panic!("Unexpected character {char}"),
                })
                .collect()
        })
        .collect();
    benchmark(|| puzzle_1(&map, start, end), TimeUnit::MILLISECONDS);
    benchmark(|| puzzle_2(&map, start, end), TimeUnit::MILLISECONDS);
}
