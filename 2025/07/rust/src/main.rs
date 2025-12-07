use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    Beam,
    Splitter,
    Start,
}

type Grid = Vec<Vec<Tile>>;

fn puzzle1(grid: &Grid) {
    let mut grid = grid.clone();
    fn continue_beam_move(position: (usize, usize), grid: &mut Grid) -> u64 {
        let (x, y) = position;
        let y = y + 1;
        if y == grid.len() {
            return 0;
        }
        match grid[y][x] {
            Tile::Empty => {
                grid[y][x] = Tile::Beam;
                continue_beam_move((x, y), grid)
            }
            Tile::Beam => 0,
            Tile::Splitter => {
                let left_move = match x.checked_sub(1) {
                    Some(lx) => continue_beam_move((lx, y), grid),
                    None => 0,
                };
                let right_move = match x + 1 {
                    rx if rx < grid[y].len() => continue_beam_move((rx, y), grid),
                    _ => 0,
                };
                1 + left_move + right_move
            }
            Tile::Start => panic!("Start should not appear"),
        }
    }

    let (start_point, _) = grid[0]
        .iter()
        .enumerate()
        .find(|(_, tile)| **tile == Tile::Start)
        .unwrap();
    let splits = continue_beam_move((start_point, 1), &mut grid);
    println!("[Puzzle 1] There are {splits} splits in total");
}

fn puzzle2(grid: &Grid) {
    let mut grid = grid.clone();
    fn continue_beam_move(
        position: (usize, usize),
        grid: &mut Grid,
        cache: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        let (x, y) = position;
        let y = y + 1;
        if y == grid.len() {
            return 1;
        }
        if let Some(cached_result) = cache.get(&(x, y)) {
            return *cached_result;
        }
        let result = match grid[y][x] {
            Tile::Empty => {
                grid[y][x] = Tile::Beam;
                let paths = continue_beam_move((x, y), grid, cache);
                grid[y][x] = Tile::Empty;
                paths
            }
            Tile::Splitter => {
                let left_paths = match x.checked_sub(1) {
                    Some(lx) => continue_beam_move((lx, y), grid, cache),
                    None => 0,
                };
                let right_paths = match x + 1 {
                    rx if rx < grid[y].len() => continue_beam_move((rx, y), grid, cache),
                    _ => 0,
                };
                left_paths + right_paths
            }
            Tile::Start | Tile::Beam => panic!("Start or Bean tiles should not appear!"),
        };
        cache.insert((x, y), result);
        result
    }

    let (start_point, _) = grid[0]
        .iter()
        .enumerate()
        .find(|(_, tile)| **tile == Tile::Start)
        .unwrap();

    let splits = continue_beam_move((start_point, 1), &mut grid, &mut HashMap::new());
    println!("[Puzzle 2] There are {splits} possible paths to the end");
}

fn main() {
    let grid = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Tile::Empty,
                    '^' => Tile::Splitter,
                    'S' => Tile::Start,
                    _ => panic!("Unexpected input char {char}"),
                })
                .collect()
        })
        .collect();
    puzzle1(&grid);
    puzzle2(&grid);
}
