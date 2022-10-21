use crate::error::{self, MatrixError};
use crate::num::{One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

mod index;
mod iter;
mod ops;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    pub(crate) row: usize,
    pub(crate) column: usize,
    pub(crate) entries: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Add + AddAssign + Mul + MulAssign + Sub + SubAssign + Div + DivAssign + Zero + One + Clone,
{
    pub fn new(
        num_row: usize,
        num_column: usize,
        buffer: Vec<T>,
    ) -> Result<Self, error::MatrixError> {
        let num_of_entries = num_row * num_column;

        if num_of_entries == buffer.len() {
            Ok(Matrix {
                row: num_row,
                column: num_column,
                entries: buffer,
            })
        } else {
            let error = format!("{}!={}", num_of_entries, buffer.len());
            Err(MatrixError::new(&error.clone()))
        }
    }

    pub fn eye(dimension: usize) -> Result<Self, error::MatrixError> {
        match dimension {
            0 => Err(MatrixError::new("Dimension can't be zero")),
            _ => {
                let num_of_elem: usize = dimension * dimension;
                let mut buffer: Vec<T> = Vec::<T>::with_capacity(num_of_elem);

                for i in 0..dimension {
                    for j in 0..dimension {
                        if i != j {
                            buffer.push(T::zero());
                        } else {
                            buffer.push(T::one());
                        }
                    }
                }

                Ok(Matrix {
                    row: dimension,
                    column: dimension,
                    entries: buffer,
                })
            }
        }
    }

    pub fn zero(num_row: usize, num_column: usize) -> Self {
        let buffer = vec![T::zero(); num_row * num_column];

        Matrix {
            row: num_row,
            column: num_column,
            entries: buffer,
        }
    }

    pub fn get_vec(&self) -> Vec<T> {
        self.entries.clone()
    }

    pub fn index_calc(
        dimension: (usize, usize),
        row: usize,
        column: usize,
    ) -> Result<usize, error::MatrixError> {
        if dimension.0 < row || dimension.1 < column {
            let error = format!(
                "It is not possible to index [{}][{}]\n
                                        if the Matrix is a {}x{}!",
                row, column, dimension.0, dimension.1
            );
            Err(MatrixError::new(&error.clone()))
        } else {
            Ok(row * dimension.0 + column)
        }
    }
}
