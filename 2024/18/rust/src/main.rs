use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::fs::read_to_string;
use std::u64;

#[derive(Clone, PartialEq)]
enum Tile {
    Path,
    Wall,
}

type Maze = Vec<Vec<Tile>>;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    fn neighours(&self, maze: &Maze) -> Vec<(usize, usize)> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        directions
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = self.x as i64 + dx;
                let ny = self.y as i64 + dy;
                if nx >= 0
                    && ny >= 0
                    && (ny as usize) < maze.len()
                    && (nx as usize) < maze[ny as usize].len()
                {
                    if maze[ny as usize][nx as usize] == Tile::Path {
                        return Some((nx as usize, ny as usize));
                    }
                }
                None
            })
            .collect()
    }
}

struct MazeSolver {
    unvisited_nodes: PriorityQueue<Node, Reverse<u64>>,
}

impl MazeSolver {
    fn new() -> Self {
        MazeSolver {
            unvisited_nodes: PriorityQueue::new(),
        }
    }
    fn solve(mut self, maze: &Maze, begin: Node, end: Node) -> Option<u64> {
        for y in 0..maze.len() {
            for x in 0..maze[y].len() {
                let distance = if x == begin.x && y == begin.y {
                    0
                } else {
                    u64::MAX
                };
                self.unvisited_nodes.push(Node { x, y }, Reverse(distance));
            }
        }

        while let Some((node, distance)) = self.unvisited_nodes.pop() {
            let distance = distance.0;
            if distance == u64::MAX {
                continue;
            }
            if node == end {
                return Some(distance);
            }

            let neighbours = node.neighours(maze);
            for neighbour in neighbours {
                let (nx, ny) = neighbour;
                let new_distance = distance + 1;
                if let Some((_, distance)) = self.unvisited_nodes.get(&Node { x: nx, y: ny }) {
                    if distance.0 > new_distance {
                        self.unvisited_nodes
                            .push(Node { x: nx, y: ny }, Reverse(new_distance));
                    }
                }
            }
        }
        None
    }
}

fn puzzle_1() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut maze = vec![vec![Tile::Path; 71]; 71];
    file_content
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .take(1024)
        .for_each(|(x, y)| {
            maze[y][x] = Tile::Wall;
        });
    let maze_solver = MazeSolver::new();
    let distance = maze_solver
        .solve(&maze, Node { x: 0, y: 0 }, Node { x: 70, y: 70 })
        .unwrap();
    println!("Closest distance through the maze is {distance}");
}

fn puzzle_2() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut i = 1;
    loop {
        let mut maze = vec![vec![Tile::Path; 71]; 71];
        file_content
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .take(i)
            .for_each(|(x, y)| {
                maze[y][x] = Tile::Wall;
            });
        let maze_solver = MazeSolver::new();
        let distance = maze_solver
            .solve(&maze, Node { x: 0, y: 0 }, Node { x: 70, y: 70 });
            
        if distance.is_none() {
            println!("There is no solution when {} bytes are corrupted", i);
            break;
        } else {
            println!("{i} | {}", distance.unwrap());
        }
        i += 1;
    }
}

fn main() {
    puzzle_1();
    puzzle_2();
}
