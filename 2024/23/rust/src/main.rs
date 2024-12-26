use common::benchmark::{benchmark, TimeUnit};
use std::thread::{self, JoinHandle};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Clone)]
struct Computer {
    address: String,
    neighbours: Vec<String>,
}

#[derive(Clone)]
struct Network {
    computers: HashMap<String, Computer>,
}

impl Network {
    fn new() -> Self {
        Network {
            computers: HashMap::new(),
        }
    }
    fn add_of_modify(&mut self, computer_address: &str, neigbour_address: &str) {
        self.computers
            .entry(computer_address.to_string())
            .and_modify(|computer| computer.neighbours.push(neigbour_address.to_string()))
            .or_insert_with(|| Computer {
                address: computer_address.to_string(),
                neighbours: vec![neigbour_address.to_string()],
            });
    }

    fn connect_computers(&mut self, from: &str, to: &str) {
        self.add_of_modify(from, to);
        self.add_of_modify(to, from);
    }

    fn cycles_dfs(
        &self,
        initial: &Computer,
        current: &Computer,
        path_so_far: &mut Vec<String>,
        length: usize,
        results: &mut Vec<Vec<String>>,
    ) {
        if path_so_far.len() == length {
            if current.neighbours.contains(&initial.address) {
                results.push(path_so_far.clone());
            }
            return;
        }
        for neighbour_address in &current.neighbours {
            if path_so_far.contains(neighbour_address) {
                continue;
            }
            path_so_far.push(neighbour_address.clone());
            let neighour = &self.computers[neighbour_address];
            self.cycles_dfs(initial, neighour, path_so_far, length, results);
            path_so_far.pop();
        }
    }

    fn find_cycles_from_address(&self, from: &str, length: usize) -> Vec<Vec<String>> {
        let mut all_cycles = Vec::new();
        let initial = &self.computers[&from.to_string()];
        self.cycles_dfs(
            initial,
            initial,
            &mut vec![initial.address.clone()],
            length,
            &mut all_cycles,
        );
        all_cycles
    }

    fn clique_dfs(
        &self,
        clique: &mut HashSet<String>,
        candidates: &Vec<String>,
        biggest: &mut Vec<String>,
    ) {
        let mut went_deeper = false;
        'candidates: for candidate in candidates {
            if clique.contains(candidate) {
                continue;
            }
            for clique_member in clique.iter() {
                if !self.computers[clique_member].neighbours.contains(candidate) {
                    continue 'candidates;
                }
            }
            clique.insert(candidate.clone());
            self.clique_dfs(clique, candidates, biggest);
            clique.remove(candidate);
            went_deeper = true;
        }
        if !went_deeper {
            if biggest.len() < clique.len() {
                biggest.clear();
                biggest.extend(clique.iter().map(|s| s.clone()));
            }
        }
    }

    fn find_clique(&self) {
        let handles = self
            .computers
            .keys()
            .filter(|address| address.starts_with("t"))
            .map(|address| {
                let address = address.clone();
                let self_copy = self.clone();
                thread::spawn(move || {
                    let mut clique = HashSet::from([address.clone()]);
                    let candidates = &self_copy.computers[&address].neighbours;
                    let mut biggest = Vec::new();
                    self_copy.clique_dfs(&mut clique, candidates, &mut biggest);
                    println!("Final result {:?} {}", biggest, biggest.len());
                    biggest
                })
            })
            .collect::<Vec<JoinHandle<Vec<String>>>>();
        let mut biggest_result = Vec::new();
        for handle in handles {
            let result = handle.join().unwrap();
            if biggest_result.len() < result.len() {
                biggest_result = result;
            }
        }
        biggest_result.sort();
        let password = biggest_result.join(",");
        println!("The password is {}", password);
    }
}

fn puzzle_1(network: &Network) {
    let mut computer_tuples = HashSet::new();
    for starting_computer in network.computers.keys() {
        let results = network.find_cycles_from_address(starting_computer, 3);
        for mut result in results {
            if result.iter().any(|address| address.starts_with("t")) {
                result.sort();
                computer_tuples.insert(result);
            }
        }
    }
    println!(
        "There are {} sets of inter-connected computers that contain computer starting with t",
        computer_tuples.len()
    );
}

fn puzzle_2(network: &Network) {
    network.find_clique();
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let mut network = Network::new();
    file_content
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .for_each(|(left, right)| {
            network.connect_computers(left, right);
        });
    puzzle_1(&network);
    benchmark(|| puzzle_2(&network), TimeUnit::SECONDS);
}
