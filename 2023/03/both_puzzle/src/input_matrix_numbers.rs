use super::found_number::FoundNumber;
use super::found_star::FoundStar;

pub struct InputMatrixNumbers {
    pub numbers: Vec<Vec<FoundNumber>>,
}

impl InputMatrixNumbers {
    pub fn all(&self) -> Vec<&FoundNumber> {
        self.numbers.iter().flatten().collect()
    }

    pub fn find_two_numbers_next_to_star(
        &self,
        star: &FoundStar,
    ) -> Option<(&FoundNumber, &FoundNumber)> {
        let candidates: Vec<&FoundNumber> = self.numbers
            [star.line as usize - 1..=star.line as usize + 1]
            .iter()
            .flat_map(|line| line.iter())
            .filter(|number| number.bounding_rect().contains(&(star.line, star.column)))
            .collect();

        match candidates.len() {
            2 => Some((&candidates[0], &candidates[1])),
            _ => None,
        }
    }
}

#[test]
fn test_find_two_numbers_next_to_star() {
    let input_matrix_numbers = InputMatrixNumbers {
        numbers: vec![
            Vec::new(),
            vec![
                FoundNumber {
                    number: 1,
                    line: 1,
                    column: 1,
                    number_length: 1,
                },
                FoundNumber {
                    number: 2,
                    line: 1,
                    column: 3,
                    number_length: 1,
                },
                FoundNumber {
                    number: 3,
                    line: 1,
                    column: 4,
                    number_length: 1,
                },
            ],
            Vec::new(),
        ],
    };

    assert!(input_matrix_numbers
        .find_two_numbers_next_to_star(&FoundStar { line: 1, column: 2 })
        .is_some());
    assert!(input_matrix_numbers
        .find_two_numbers_next_to_star(&FoundStar { line: 1, column: 5 })
        .is_none());
}
