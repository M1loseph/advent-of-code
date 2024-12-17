use std::{collections::{HashMap, HashSet}, fs::read_to_string};

enum Tile {
    Wall,
    Floor,
}

#[derive(PartialEq)]
enum Facing {
    Left,
    Right,
    Up,
    Down,
}

impl Facing {
    fn to_offset(&self) -> (isize, isize) {
        match self {
            Facing::Left => (-1, 0),
            Facing::Right => (1, 0),
            Facing::Up => (0, -1),
            Facing::Down => (0, 1),
        }
    }
}

struct Maze {
    start: (usize, usize),
    end: (usize, usize),
    maze: Vec<Vec<Tile>>,
}

struct Node<'a> {
    heuristic_cost: u64,
    score: u64,
    x: usize,
    y: usize,
    parent: Option<&'a Node<'a>>,
    direction: Facing,
}

struct MazeSolver<'a> {
    maze: &'a Maze,
    visited_nodes: HashMap<(usize, usize), u64>,
    tiles: HashSet<(usize, usize)>,
}

impl<'a> MazeSolver<'a> {
    fn new(maze: &'a Maze) -> Self {
        MazeSolver {
            maze,
            visited_nodes: HashMap::new(),
            tiles: HashSet::new(),
        }
    }

    // TODO: create a true A* from this recursion
    fn recurr(&mut self, node: &Node, best_solution: &mut Option<u64>) {
        let score = node.score;
        if (node.x, node.y) == self.maze.end {
            if best_solution.is_none() || score < best_solution.unwrap() {
                self.tiles.clear();
                println!("Found solution {score}");
                *best_solution = Some(score);
            }
            if best_solution.unwrap() == score {
                let mut curr = Some(node);
                while curr.is_some() {
                    self.tiles.insert((curr.unwrap().x, curr.unwrap().y));
                    curr = curr.unwrap().parent;
                }
            }
            return;
        }
        if let Some(cached_score) = self.visited_nodes.get(&(node.x, node.y)) {
            if *cached_score + 1001 < score {
                return;
            }
        }
        self.visited_nodes.insert((node.x, node.y), score);
        let directions = [Facing::Down, Facing::Up, Facing::Left, Facing::Right];
        let mut possible_child_nodes: Vec<Node> = directions
            .into_iter()
            .filter_map(|direction| {
                let (dx, dy) = direction.to_offset();
                let nx = node.x.checked_add_signed(dx).unwrap();
                let ny = node.y.checked_add_signed(dy).unwrap();
                match self.maze.maze[ny][nx] {
                    Tile::Wall => None,
                    Tile::Floor => {
                        let new_score = if node.direction != direction {
                            score + 1001
                        } else {
                            score + 1
                        };
                        let (tx, ty) = self.maze.end;
                        let g = new_score;
                        let h = tx.abs_diff(nx) as u64 + ty.abs_diff(ny) as u64;
                        let f = g + h;
                        Some(Node {
                            heuristic_cost: f,
                            score: g,
                            direction,
                            parent: Some(node),
                            x: nx,
                            y: ny,
                        })
                    }
                }
            })
            .collect();
        possible_child_nodes.sort_by(|lh, rh| lh.heuristic_cost.cmp(&rh.heuristic_cost));
        for node in possible_child_nodes {
            self.recurr(&node, best_solution);
        }
    }
}

impl Maze {
    fn find_path(&self) -> (u64, usize) {
        let mut solution = None;
        let (x, y) = self.start;
        let mut solver = MazeSolver::new(self);
        solver.recurr(
            &Node {
                heuristic_cost: 0,
                score: 0,
                x,
                y,
                parent: None,
                direction: Facing::Right,
            },
            &mut solution,
        );
        self.print(&solver.tiles);
        let max_count  = solver.tiles.len();
        (solution.unwrap(), max_count)
    }

    fn print(&self, best_paths: &HashSet<(usize, usize)>) {
        println!();
        for y in 0..self.maze.len() {
            for x in 0..self.maze[y].len() {
                match self.maze[y][x] {
                    Tile::Wall => print!("#"),
                    Tile::Floor => {
                        if best_paths.contains(&(x, y)) {
                            print!("O");
                        } else {
                            print!(".");
                        }
                    }
                }
            }
            println!();
        }
    }
}

fn puzzle_1(maze: &Maze) {
    let (score, count_of_tiles) = maze.find_path();
    println!("The lowest score is {score}");
    println!("Count of common tiles {count_of_tiles}");
}

// TODO: draw this algorithm
fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let maze_map = file_content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    'S' => {
                        start = (x, y);
                        Tile::Floor
                    }
                    'E' => {
                        end = (x, y);
                        Tile::Floor
                    }
                    _ => panic!("Unsupported char {char}"),
                })
                .collect()
        })
        .collect();
    let maze = Maze {
        start: start,
        end: end,
        maze: maze_map,
    };
    puzzle_1(&maze);
}
