mod file_reader;
mod found_number;
mod found_star;
mod input_matrix;
mod input_matrix_numbers;

use input_matrix::InputMatrix;

fn part_one(matrix: &InputMatrix) {
    let found_numbers = matrix.find_all_numbers();
    let solution: u32 = found_numbers
        .all()
        .iter()
        .filter(|n| matrix.has_edge(n))
        .map(|n| n.number)
        .sum();

    println!("Part 1 solution: {}", solution);
}

fn part_two(matrix: &InputMatrix) {
    let found_numbers = matrix.find_all_numbers();
    let found_stars = matrix.find_all_stars();

    let solution: u32 = found_stars
        .iter()
        .filter_map(|star| found_numbers.find_two_numbers_next_to_star(star))
        .map(|(n1, n2)| n1.number * n2.number)
        .sum();

    println!("Part 2 solution: {}", solution);
}

fn main() {
    let matrix = InputMatrix::read_from_file("input/input.txt").unwrap();
    part_one(&matrix);
    part_two(&matrix);
}
