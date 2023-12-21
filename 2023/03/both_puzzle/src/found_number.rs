#[derive(Debug)]
pub struct FoundNumber {
    pub number: u32,
    pub line: i32,
    pub column: i32,
    pub number_length: i32,
}

impl FoundNumber {
    pub fn push_digit(&mut self, digit: u32) {
        self.number = self.number * 10 + digit;
        self.number_length += 1;
    }

    pub fn bounding_rect(&self) -> Vec<(i32, i32)> {
        (self.line - 1..=self.line + 1)
            .map(|i| (i, self.column - 1..=self.column + self.number_length))
            .flat_map(|(line_index, column_indexes)| {
                column_indexes.map(move |column_index| (line_index, column_index))
            })
            .filter(|(x, y)| {
                !(*x == self.line && (self.column..self.column + self.number_length).contains(y))
            })
            .collect()
    }
}

#[test]
fn test_bounding_rect() {
    let found_number = FoundNumber {
        number: 123,
        line: 1,
        column: 1,
        number_length: 3,
    };

    let bounding_rect = found_number.bounding_rect();

    assert_eq!(
        bounding_rect,
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (1, 4),
            (2, 0),
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
        ]
    );
}
