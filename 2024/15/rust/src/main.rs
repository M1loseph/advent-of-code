use std::collections::VecDeque;
use std::fs::read_to_string;

const PRINT: bool = false;

struct Warehouse {
    robot_position: (usize, usize),
    map: Vec<Vec<Tile>>,
    moves: VecDeque<Move>,
}

impl Warehouse {
    fn new(map: Vec<Vec<Tile>>, moves: VecDeque<Move>) -> Self {
        let find_robot = || {
            for y in 0..map.len() {
                for x in 0..map[y].len() {
                    if map[y][x] == Tile::Robot {
                        return (x, y);
                    }
                }
            }
            panic!("Did not find robot on the map");
        };
        let robot_position = find_robot();

        Warehouse {
            robot_position: robot_position,
            map: map,
            moves: moves,
        }
    }

    fn move_in_direction(&mut self, next_move: &Move, x: usize, y: usize) {
        let (dx, dy) = next_move.delta();
        let nx = x.checked_add_signed(dx).unwrap();
        let ny = y.checked_add_signed(dy).unwrap();
        match self.map[ny][nx] {
            Tile::Box => {
                self.move_in_direction(next_move, nx, ny);
                let current = self.map[y][x].clone();
                self.map[ny][nx] = current;
                self.map[y][x] = Tile::Empty;
            }
            Tile::Empty => {
                let current = self.map[y][x].clone();
                self.map[ny][nx] = current;
                self.map[y][x] = Tile::Empty;
            }
            Tile::BoxLeft => {
                match next_move {
                    Move::Left | Move::Right => {
                        self.move_in_direction(next_move, nx, ny);
                    }
                    Move::Up | Move::Down => {
                        self.move_in_direction(next_move, nx, ny);
                        self.move_in_direction(next_move, nx + 1, ny);
                    }
                };
                let current = self.map[y][x].clone();
                self.map[ny][nx] = current;
                self.map[y][x] = Tile::Empty;
            }
            Tile::BoxRight => {
                match next_move {
                    Move::Left | Move::Right => {
                        self.move_in_direction(next_move, nx, ny);
                    }
                    Move::Up | Move::Down => {
                        self.move_in_direction(next_move, nx, ny);
                        self.move_in_direction(next_move, nx - 1, ny);
                    }
                };
                let current = self.map[y][x].clone();
                self.map[ny][nx] = current;
                self.map[y][x] = Tile::Empty;
            }
            Tile::Wall => panic!("There should be no wall on the way!"),
            Tile::Robot => panic!("There is a robot on ({nx}, {ny}) - that should not happen!"),
        }
    }

    fn can_move(&self, next_move: &Move, x: usize, y: usize) -> bool {
        let (dx, dy) = next_move.delta();
        let nx = x.checked_add_signed(dx).unwrap();
        let ny = y.checked_add_signed(dy).unwrap();
        match self.map[ny][nx] {
            Tile::Wall => false,
            Tile::Box => self.can_move(next_move, nx, ny),
            Tile::Empty => true,
            Tile::BoxLeft => match next_move {
                Move::Left | Move::Right => self.can_move(next_move, nx, ny),
                Move::Up | Move::Down => {
                    self.can_move(next_move, nx, ny) && self.can_move(next_move, nx + 1, ny)
                }
            },
            Tile::BoxRight => match next_move {
                Move::Left | Move::Right => self.can_move(next_move, nx, ny),
                Move::Up | Move::Down => {
                    self.can_move(next_move, nx, ny) && self.can_move(next_move, nx - 1, ny)
                }
            },
            Tile::Robot => panic!("There is a robot on ({nx}, {ny}) - that should not happen!"),
        }
    }

    fn try_to_move(&mut self, next_move: &Move, x: usize, y: usize) -> bool {
        if self.can_move(next_move, x, y) {
            self.move_in_direction(next_move, x, y);
            return true;
        }
        false
    }

    fn perform_simulation(&mut self) {
        while let Some(next_move) = self.moves.pop_front() {
            let (rx, ry) = self.robot_position;
            if self.try_to_move(&next_move, rx, ry) {
                let (dx, dy) = next_move.delta();
                self.robot_position = (
                    rx.checked_add_signed(dx).unwrap(),
                    ry.checked_add_signed(dy).unwrap(),
                );
            }
            self.print(Some(&next_move));
        }
    }

    fn print(&self, next_move: Option<&Move>) {
        if PRINT {
            if let Some(next_move) = next_move {
                println!("Ran move {}", next_move.to_char());
            }
            for line in &self.map {
                for tile in line {
                    print!("{}", tile.to_char());
                }
                println!();
            }
        }
    }

    fn sum_gps(&self) -> u64 {
        let mut gps_sum = 0u64;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == Tile::Box || self.map[y][x] == Tile::BoxLeft {
                    let gps = 100u64 * y as u64 + x as u64;
                    gps_sum += gps;
                }
            }
        }
        gps_sum
    }
}

enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl Move {
    const LEFT: char = '<';
    const RIGHT: char = '>';
    const DOWN: char = 'v';
    const UP: char = '^';

    fn form_char(char: char) -> Self {
        match char {
            Move::LEFT => Move::Left,
            Move::RIGHT => Move::Right,
            Move::DOWN => Move::Down,
            Move::UP => Move::Up,
            _ => panic!("Unexpected char {char} that is not a direaction"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Move::Left => Move::LEFT,
            Move::Right => Move::RIGHT,
            Move::Up => Move::UP,
            Move::Down => Move::DOWN,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
            Move::Up => (0, -1),
            Move::Down => (0, 1),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Tile {
    Robot,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Empty,
}

impl Tile {
    const WALL: char = '#';
    const ROBOT: char = '@';
    const BOX: char = 'O';
    const EMPTY: char = '.';
    const BOX_LEFT: char = '[';
    const BOX_RIGHT: char = ']';

    fn from_char(char: char) -> Self {
        match char {
            Tile::WALL => Tile::Wall,
            Tile::ROBOT => Tile::Robot,
            Tile::BOX => Tile::Box,
            Tile::EMPTY => Tile::Empty,
            _ => {
                panic!("Got char {char} that is not convertable to Tile")
            }
        }
    }

    fn from_char_v2(char: char) -> [Self; 2] {
        match char {
            Tile::WALL => [Tile::Wall, Tile::Wall],
            Tile::ROBOT => [Tile::Robot, Tile::Empty],
            Tile::BOX => [Tile::BoxLeft, Tile::BoxRight],
            Tile::EMPTY => [Tile::Empty, Tile::Empty],
            _ => {
                panic!("Got char {char} that is not convertable to Tile")
            }
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Robot => Tile::ROBOT,
            Tile::Wall => Tile::WALL,
            Tile::Box => Tile::BOX,
            Tile::Empty => Tile::EMPTY,
            Tile::BoxLeft => Tile::BOX_LEFT,
            Tile::BoxRight => Tile::BOX_RIGHT,
        }
    }
}

fn puzzle_1(map_input: &Vec<String>, moves_input: &Vec<String>) {
    let moves = moves_input
        .iter()
        .flat_map(|line| line.chars().map(|char| Move::form_char(char)))
        .collect();
    let map = map_input
        .iter()
        .map(|line| line.chars().map(|char| Tile::from_char(char)).collect())
        .collect();

    let mut warehouse = Warehouse::new(map, moves);
    warehouse.print(None);
    warehouse.perform_simulation();
    let gps_sum = warehouse.sum_gps();
    println!("Sum of all GPS coordinates is {gps_sum}");
}

fn puzzle_2(map_input: &Vec<String>, moves_input: &Vec<String>) {
    let moves = moves_input
        .iter()
        .flat_map(|line| line.chars().map(|char| Move::form_char(char)))
        .collect();
    let map = map_input
        .iter()
        .map(|line| {
            line.chars()
                .flat_map(|char| Tile::from_char_v2(char))
                .collect()
        })
        .collect();

    let mut warehouse = Warehouse::new(map, moves);
    warehouse.print(None);
    warehouse.perform_simulation();
    let gps_sum = warehouse.sum_gps();
    println!("Sum of all GPS coordinates is {gps_sum}");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let map_file_part = file_content
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|str| str.to_string())
        .collect();
    let robot_moves_file_part = file_content
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|str| str.to_string())
        .collect();

    puzzle_1(&map_file_part, &robot_moves_file_part);
    puzzle_2(&map_file_part, &robot_moves_file_part);
}
