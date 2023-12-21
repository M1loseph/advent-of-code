mod filereader;
mod found_number;
mod input_matrix;

use input_matrix::InputMatrix;

fn main() {
    let matrix = InputMatrix::read_from_file("input/input.txt").unwrap();
    let found_numbers = matrix.find_all_numbers();

    let solution: u32 = found_numbers
        .iter()
        .filter(|n| matrix.has_edge(n))
        .map(|n| n.number)
        .sum();

    println!("Solution: {}", solution);
}
