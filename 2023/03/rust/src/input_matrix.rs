use crate::file_reader;
use crate::found_number::FoundNumber;
use crate::found_star::FoundStar;
use crate::input_matrix_numbers::InputMatrixNumbers;


pub struct InputMatrix {
    matrix: Vec<Vec<char>>,
}

impl InputMatrix {
    pub fn read_from_file(file_path: &str) -> std::io::Result<InputMatrix> {
        let lines = file_reader::file_reader::read_input_file(file_path)?
            .filter_map(|line| line.ok())
            .map(|line| line.chars().collect())
            .collect();
        Ok(InputMatrix { matrix: lines })
    }

    pub fn find_all_numbers(&self) -> InputMatrixNumbers {
        let numbers = self
            .matrix
            .iter()
            .enumerate()
            .map(|(line_index, line)| {
                let mut all_numbers = Vec::new();
                let mut result: Option<FoundNumber> = None;
                for (i, char) in line.iter().chain(std::iter::once(&'.')).enumerate() {
                    match char.to_digit(10) {
                        Some(digit) => {
                            result = match result {
                                Some(mut found_number_so_far) => {
                                    found_number_so_far.push_digit(digit);
                                    Some(found_number_so_far)
                                }
                                None => Some(FoundNumber {
                                    number: digit,
                                    line: line_index as i32,
                                    column: i as i32,
                                    number_length: 1,
                                }),
                            };
                        }
                        None => match result {
                            Some(found_number) => {
                                all_numbers.push(found_number);
                                result = None;
                            }
                            None => (),
                        },
                    };
                }
                all_numbers
            })
            .collect();
        InputMatrixNumbers { numbers }
    }

    pub fn find_all_stars(&self) -> Vec<FoundStar> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(line_index, chars)| {
                chars
                    .iter()
                    .enumerate()
                    .filter(|(_, char)| **char == '*')
                    .map(move |(column, _)| FoundStar {
                        line: line_index as i32,
                        column: column as i32,
                    })
            })
            .collect()
    }

    pub fn has_edge(&self, found_number: &FoundNumber) -> bool {
        found_number
            .bounding_rect()
            .iter()
            .any(|(x, y)| match self.matrix.get(*x as usize) {
                Some(line) => match line.get(*y as usize) {
                    Some(character) => *character != '.',
                    None => false,
                },
                None => false,
            })
    }
}
