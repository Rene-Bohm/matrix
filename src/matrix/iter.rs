/*
pub struct MatrixIntoIterator<T> {
    matrix: Matrix<T>,
    index_row: usize,
    index_column: usize,
}

impl Iterator for MatrixIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let dimension = self.matrix.dimension();

        if (self.index_column < dimension.1) {
            let result = self.matrix[self.index_row][self.index_column];
            self.index_column += 1;
            Some(result)
        } else if (self.index_row < dimension.0 - 1) {
            self.index_column = 0;
            self.index_row += 1;
            self.next()
        } else {
            None
        }
    }
}

impl IntoIterator for Matrix<T> {
    type Item = f64;
    type IntoIter = MatrixIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIntoIterator {
            matrix: self,
            index_row: 0,
            index_column: 0,
        }
    }
}
*/
