use super::Matrix;
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[(index * self.row)..(index + 1) * self.row]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [T] {
        &mut self.entries[(index * self.row)..(index + 1) * self.row]
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.entries[index.0 * self.row + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.entries[index.0 * self.row + index.1]
    }
}
