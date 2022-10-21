use matrix::matrix::Matrix;

#[test]
fn index() {
    let buffer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let matrix_new = Matrix::new(3, 3, buffer).unwrap();

    let row = &matrix_new[2];
    let elem = matrix_new[1][0];

    assert_eq!(row, vec![7, 8, 9]);
    assert_eq!(elem, 4);
}

#[test]
fn index_mut() {
    let buffer = vec![1, 2, 3, 0, 0, 0, 7, 8, 0];

    let mut matrix_new = Matrix::new(3, 3, buffer).unwrap();

    //matrix_new.unwrap()[1] = &[4,5,6];
    matrix_new[(2, 2)] = 9;

    println!("{:?}", matrix_new);
}
