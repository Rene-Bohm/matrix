use std::ops::{Index, IndexMut};

use super::Matrix;
/*
impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.entries[index.0][index.1]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [T] {
        &mut self.entries[index]
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &[f64] {
        &self.entries[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut &self.entries[index.0][index.1]
    }
}
*/
