#[cfg(test)]
mod test {
    use crate::{error::MatrixError, matrix::Matrix};

    #[test]
    fn new() {
        let buffer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let matrix_new = Matrix::new(3, 3, buffer);

        assert_eq!(matrix_new.unwrap().get_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let buffer = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let error = format!("{}!={}", 9, &buffer.len());

        let matrix_new = Matrix::new(3, 3, buffer);

        assert_eq!(matrix_new, Err(MatrixError::new(&error)));
    }

    #[test]
    fn eye() {
        let buffer = vec![1, 0, 0, 0, 1, 0, 0, 0, 1];

        let matrix_new = Matrix::new(3, 3, buffer);

        let matrix_eye = Matrix::<i32>::eye(3);

        assert_eq!(matrix_eye.unwrap(), matrix_new.unwrap());

        let matrix_eye = Matrix::<i32>::eye(0);

        assert_eq!(matrix_eye, Err(MatrixError::new("Dimension can't be zero")));
    }

    #[test]
    fn zero() {
        let buffer = vec![0; 9];
        let matrix_new = Matrix::new(3, 3, buffer);

        let matrix_zero = Matrix::<i32>::zero(3, 3);

        assert_eq!(matrix_new.unwrap(), matrix_zero);
    }
}
