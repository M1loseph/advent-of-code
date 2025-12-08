use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

struct Distance {
    from: usize,
    to: usize,
    distance: f64,
}

#[derive(Debug, Clone)]
struct AdjacencyMatrixCell {
    connected: bool,
}

impl Point3D {
    fn distance(&self, other: &Point3D) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

fn dfs(
    index: usize,
    adjacency_matrix: &Vec<Vec<AdjacencyMatrixCell>>,
    visited: &mut HashSet<usize>,
) -> u64 {
    if visited.contains(&index) {
        return 0;
    }
    visited.insert(index);
    let connections: u64 = adjacency_matrix[index]
        .iter()
        .enumerate()
        .map(|(index, e)| {
            if e.connected {
                dfs(index, adjacency_matrix, visited)
            } else {
                0
            }
        })
        .sum();
    connections + 1
}

fn distances_between_all_points_asc(points: &Vec<Point3D>) -> Vec<Distance> {
    let mut distances: Vec<Distance> = vec![];
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            distances.push(Distance {
                from: i,
                to: j,
                distance: points[i].distance(&points[j]),
            });
        }
    }
    distances.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    distances
}

fn puzzle1(points: &Vec<Point3D>, distances: &Vec<Distance>) {
    let mut adjacency_matrix =
        vec![vec![AdjacencyMatrixCell { connected: false }; points.len()]; points.len()];
    for i in 0..1000 {
        let closest = &distances[i];
        adjacency_matrix[closest.from][closest.to].connected = true;
        adjacency_matrix[closest.to][closest.from].connected = true;
    }
    let mut visited = HashSet::new();
    let mut sizes = vec![];
    for i in 0..adjacency_matrix.len() {
        let result = dfs(i, &adjacency_matrix, &mut visited);
        if result != 0 {
            sizes.push(result);
        }
    }
    sizes.sort();
    let result = sizes.iter().rev().take(3).fold(1u64, |acc, e| acc * e);
    println!("[Puzzle 1] Result: {result}")
}

fn puzzle2(points: &Vec<Point3D>, distances: &Vec<Distance>) {
    let mut adjacency_matrix =
        vec![vec![AdjacencyMatrixCell { connected: false }; points.len()]; points.len()];

    for i in 0..distances.len() {
        let closest = &distances[i];
        adjacency_matrix[closest.from][closest.to].connected = true;
        adjacency_matrix[closest.to][closest.from].connected = true;

        let mut visited = HashSet::new();
        let mut sizes = vec![];
        for i in 0..adjacency_matrix.len() {
            let result = dfs(i, &adjacency_matrix, &mut visited);
            if result != 0 {
                sizes.push(result);
            }
        }
        if sizes.len() == 1 {
            println!(
                "[Puzzle 2] Result: {}",
                points[closest.from].x * points[closest.to].x
            );
            break;
        }
    }
}

fn main() {
    let points: Vec<Point3D> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|point| {
            let mut coordinates = point.splitn(3, ",");
            Point3D {
                x: coordinates.next().unwrap().parse().unwrap(),
                y: coordinates.next().unwrap().parse().unwrap(),
                z: coordinates.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    let distances = distances_between_all_points_asc(&points);
    puzzle1(&points, &distances);
    puzzle2(&points, &distances);
}
