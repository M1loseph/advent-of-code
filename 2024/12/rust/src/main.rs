use std::fs::read_to_string;

struct Garden {
    plots: Vec<Vec<char>>,
}

struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Facing {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Clone, Copy, Debug)]
struct Fence {
    x: usize,
    y: usize,
    facing: Facing,
}

impl Garden {
    fn pelimeter(fences: Vec<Fence>) -> u64 {
        fences.len() as u64
    }

    fn sides(mut fences: Vec<Fence>) -> u64 {
        let mut sides = 0;

        fn find_recurr(current: Fence, fences: &mut Vec<Fence>) {
            let mut next_parts: Vec<usize> = fences
                .iter()
                .enumerate()
                .filter(|(_, fence)| {
                    if fence.facing != current.facing {
                        return false;
                    }
                    match fence.facing {
                        Facing::UP | Facing::DOWN => {
                            current.y == fence.y && current.x.abs_diff(fence.x) == 1
                        }
                        Facing::LEFT | Facing::RIGHT => {
                            current.x == fence.x && current.y.abs_diff(fence.y) == 1
                        }
                    }
                })
                .map(|(i, _)| i)
                .collect();
            next_parts.sort();
            match next_parts.len() {
                0 => {}
                1 => {
                    find_recurr(fences.remove(next_parts[0]), fences);
                }
                2 => {
                    // don't change the order
                    let second = fences.remove(next_parts[1]);
                    let first = fences.remove(next_parts[0]);

                    find_recurr(first, fences);
                    find_recurr(second, fences);
                }
                _ => {
                    panic!(
                        "Found {} neighour parts, while it should not be possible",
                        next_parts.len()
                    );
                }
            }
        }
        while !fences.is_empty() {
            sides += 1;
            let current = fences.pop().unwrap();
            find_recurr(current, &mut fences);
        }
        sides
    }

    fn dfs(
        &self,
        searched: char,
        x: usize,
        y: usize,
        visited: &mut Vec<Point>,
        all_visited: &mut Vec<Vec<bool>>,
    ) {
        if all_visited[y][x] {
            return;
        }
        if self.plots[y][x] != searched {
            return;
        }
        all_visited[y][x] = true;
        visited.push(Point { x, y });

        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (dx, dy) in directions {
            let new_x = x.checked_add_signed(dx);
            let new_y = y.checked_add_signed(dy);
            match (new_x, new_y) {
                (Some(new_x), Some(new_y)) => {
                    if new_y < self.plots.len() && new_x < self.plots[new_y].len() {
                        self.dfs(searched, new_x, new_y, visited, all_visited);
                    }
                }
                _ => {}
            }
        }
    }

    // I'm not proud of this but it get the job done
    fn calculate_cost(&self, perimeter_argorith: fn(Vec<Fence>) -> u64) -> u64 {
        let mut visited = self
            .plots
            .iter()
            .map(|line| line.iter().map(|_| false).collect())
            .collect::<Vec<Vec<bool>>>();

        let mut total_cost = 0u64;

        for y in 0..self.plots.len() {
            for x in 0..self.plots[y].len() {
                if !visited[y][x] {
                    let mut points = Vec::new();
                    self.dfs(self.plots[y][x], x, y, &mut points, &mut visited);
                    if points.len() > 0 {
                        let area = points.len();
                        let fences = points
                            .iter()
                            .flat_map(|point| {
                                let x = point.x;
                                let y = point.y;
                                let directions =
                                    [Facing::DOWN, Facing::UP, Facing::LEFT, Facing::RIGHT];
                                directions.into_iter().filter_map(move |facing| {
                                    let (dx, dy) = match facing {
                                        Facing::LEFT => (-1, 0),
                                        Facing::RIGHT => (1, 0),
                                        Facing::UP => (0, -1),
                                        Facing::DOWN => (0, 1),
                                    };
                                    let nx = x.checked_add_signed(dx);
                                    let ny = y.checked_add_signed(dy);
                                    // TODO: try to hide this neted ugly addition
                                    match (nx, ny) {
                                        (Some(nx), Some(ny)) => {
                                            if ny < self.plots.len() {
                                                if nx < self.plots[ny].len() {
                                                    if self.plots[ny][nx] == self.plots[y][x] {
                                                        return None;
                                                    }
                                                }
                                            }
                                            Some(Fence {
                                                x,
                                                y,
                                                facing: facing,
                                            })
                                        }
                                        _ => Some(Fence {
                                            x,
                                            y,
                                            facing: facing,
                                        }),
                                    }
                                })
                            })
                            .collect();
                        let perimeter = perimeter_argorith(fences);
                        total_cost += area as u64 * perimeter;
                    }
                }
            }
        }
        total_cost
    }
}

fn puzzle_1(gardern: &Garden) {
    let total_cost = gardern.calculate_cost(Garden::pelimeter);
    println!("Fence total cost is {total_cost} using perimeter algorithm");
}

fn puzzle_2(gardern: &Garden) {
    let total_cost = gardern.calculate_cost(Garden::sides);
    println!("Fence total cost is {total_cost} using sides algorithm");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let plots = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let garden = Garden { plots };
    puzzle_1(&garden);
    puzzle_2(&garden);
}
