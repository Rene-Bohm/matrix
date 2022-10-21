use super::Matrix;
use std::ops::{Index, IndexMut};

impl<T> Matrix<T> {
    // pub fn index_calc(
    //     dimension: (usize, usize),
    //     row: usize,
    //     column: usize,
    // ) -> Result<usize, error::MatrixError> {
    //     if dimension.0 < row || dimension.1 < column {
    //         let error = format!(
    //             "It is not possible to index [{}][{}]\n
    //                                     if the Matrix is a {}x{}!",
    //             row, column, dimension.0, dimension.1
    //         );
    //         Err(MatrixError::new(&error.clone()))
    //     } else {
    //         Ok(row * dimension.0 + column)
    //     }
    // }

    pub fn get(&self, index: (usize, usize)) -> Option<&T> {
        if index.0 > self.row || index.1 > self.column {
            None
        } else {
            unsafe { Some(self.get_unchecked(index)) }
        }
    }

    pub unsafe fn get_unchecked(&self, index: (usize, usize)) -> &T {
        &self.entries[index.0 * self.row + index.1]
    }

    pub fn get_mut(&mut self, index: (usize, usize)) -> Option<&mut T> {
        if index.0 > self.row || index.1 > self.column {
            None
        } else {
            unsafe { Some(self.get_mut_unchecked(index)) }
        }
    }

    pub unsafe fn get_mut_unchecked(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.entries[index.0 * self.row + index.1]
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        unsafe { self.get_unchecked(index) }
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        unsafe { self.get_mut_unchecked(index) }
    }
}

//

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[(index * self.row)..(index + 1) * self.row]
    }
}

// impl<T> IndexMut<usize> for Matrix<T> {
//     fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [T] {
//         &mut self.entries[(index * self.row)..(index + 1) * self.row]
//     }
// }
