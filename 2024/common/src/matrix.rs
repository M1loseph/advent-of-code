use std::{
    error::Error,
    fmt::Display,
    ops::{Index, Mul},
    result::Result,
};

#[derive(Debug)]
pub enum DeterminantError {
    IsNotSquare,
}

impl Display for DeterminantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeterminantError::IsNotSquare => {
                write!(f, "Can't calculate determinant for non-square matrix")
            }
        }
    }
}

impl Error for DeterminantError {}

#[derive(Debug)]
pub enum MatrixCreationError {
    ZeroRows,
    ZeroCols,
    UnevenRows {
        reference: usize,
        incorrect_length_row_index: usize,
    },
}

impl Display for MatrixCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixCreationError::ZeroRows => write!(f, "Provided matrix has height of zero"),
            MatrixCreationError::ZeroCols => write!(f, "First matrix row length is zero"),
            MatrixCreationError::UnevenRows {
                reference,
                incorrect_length_row_index: incorrect_length_index,
            } => write!(
                f,
                "Rows {reference} and {incorrect_length_index} have differente lengths"
            ),
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut new_matrix_buffer = Vec::new();
        for ly in 0..self.rows {
            for rx in 0..rhs.cols {
                let mut acc = 0f64;
                for lx in 0..self.cols {
                    acc += self[ly][lx] * rhs[lx][rx]
                }
                new_matrix_buffer.push(acc);
            }
        }
        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            inner: new_matrix_buffer,
        }
    }
}

impl Error for MatrixCreationError {}

// TODO: make it work with any number
#[derive(Debug, PartialEq)]
pub struct Matrix {
    cols: usize,
    rows: usize,
    inner: Vec<f64>,
}

impl Index<usize> for Matrix {
    type Output = [f64];

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        return &self.inner[index * self.cols..index * self.cols + self.cols];
    }
}

impl Matrix {
    pub fn new(matrix: &[&[f64]]) -> Result<Self, MatrixCreationError> {
        let rows = matrix.len();
        if rows == 0 {
            return Err(MatrixCreationError::ZeroRows);
        }
        let cols = matrix[0].len();
        if cols == 0 {
            return Err(MatrixCreationError::ZeroCols);
        }
        let matrix_buffer: Result<Vec<f64>, MatrixCreationError> = matrix
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                if row.len() != cols {
                    vec![Err(MatrixCreationError::UnevenRows {
                        reference: 0,
                        incorrect_length_row_index: y,
                    })]
                } else {
                    row.iter().map(|element| Ok(*element)).collect()
                }
            })
            .collect();
        Ok(Matrix {
            cols,
            rows,
            inner: matrix_buffer?,
        })
    }

    pub fn determinant(&self) -> Result<f64, DeterminantError> {
        if self.cols != self.rows {
            return Err(DeterminantError::IsNotSquare);
        }
        let size = self.cols;
        return if size == 1 {
            Ok(self.inner[0])
        } else if size == 2 {
            Ok(self[0][0] * self[1][1] - self[0][1] * self[1][0])
        } else {
            todo!();
        };
    }

    // TODO: add beurifull error handling
    pub fn inverse(&self) -> Option<Matrix> {
        if self.cols != self.rows || self.cols != 2 {
            todo!();
        }
        let determinant = self.determinant().unwrap();
        if determinant == 0f64 {
            return None;
        }
        Some(
            Matrix::new(&[
                &[self[1][1] / determinant, -self[0][1] / determinant],
                &[-self[1][0] / determinant, self[0][0] / determinant],
            ])
            .unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn should_correctly_work_with_index_operator() {
        let matrix = Matrix::new(&[&[1f64, 2f64], &[3f64, 4f64]]).unwrap();

        assert_eq!(matrix[0][0], 1f64);
        assert_eq!(matrix[0][1], 2f64);
        assert_eq!(matrix[1][0], 3f64);
        assert_eq!(matrix[1][1], 4f64);
    }

    #[test]
    fn should_correcly_calculate_determinant_for_2x2_matrix() {
        {
            let matrix = Matrix::new(&[&[2f64, 3f64], &[4f64, 5f64]]).unwrap();
            assert_eq!(matrix.determinant().unwrap(), -2f64)
        }
        {
            let matrix = Matrix::new(&[&[1f64, 2f64], &[3f64, 4f64]]).unwrap();
            assert_eq!(matrix.determinant().unwrap(), -2f64)
        }
    }

    #[test]
    fn should_inverse() {
        let matrix = Matrix::new(&[&[3f64, 1f64], &[4f64, 2f64]]).unwrap();
        let inversed = matrix.inverse().unwrap();

        assert_eq!(inversed[0][0], 1f64);
        assert_eq!(inversed[0][1], -0.5f64);
        assert_eq!(inversed[1][0], -2f64);
        assert_eq!(inversed[1][1], 3f64 / 2f64);
    }

    #[test]
    fn should_not_inverse() {
        let matrix = Matrix::new(&[&[3f64, 2f64], &[6f64, 4f64]]).unwrap();

        assert_eq!(matrix.inverse(), None);
    }

    #[test]
    fn should_multiply() {
        {
            let first = Matrix::new(&[&[3f64, 1f64, 4f64]]).unwrap();
            let second = Matrix::new(&[&[4f64, 3f64], &[2f64, 5f64], &[6f64, 8f64]]).unwrap();
            let multiplication_result = first * second;

            assert_eq!(multiplication_result[0][0], 38f64);
            assert_eq!(multiplication_result[0][1], 46f64);
        }
        {
            let first = Matrix::new(&[&[1f64, 2f64], &[3f64, 4f64]]).unwrap();
            let second = Matrix::new(&[&[3f64], &[4f64]]).unwrap();
            let multiplication_result = first * second;

            assert_eq!(multiplication_result[0][0], 11f64);
            assert_eq!(multiplication_result[1][0], 25f64);
        }
    }
}
