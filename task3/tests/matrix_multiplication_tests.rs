use task3::multiply_matrices;

#[test]
fn test_matrix_multiplication() {
    let a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];

    let b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12],
    ];

    let expected = vec![
        vec![58, 64],
        vec![139, 154],
    ];

    let result = multiply_matrices(a, b, 2);

    assert_eq!(result, expected, "Matrix multiplication result is incorrect.");
}

#[test]
fn test_identity_matrix() {
    let a = vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1],
    ];

    let b = vec![
        vec![5, 6, 7],
        vec![8, 9, 10],
        vec![11, 12, 13],
    ];

    let expected = b.clone();

    let result = multiply_matrices(a, b, 2);

    assert_eq!(result, expected, "Multiplication with identity matrix failed.");
}

#[test]
fn test_zero_matrix() {
    let a = vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
    ];

    let b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12],
    ];

    let expected = vec![
        vec![0, 0],
        vec![0, 0],
    ];

    let result = multiply_matrices(a, b, 2);

    assert_eq!(result, expected, "Multiplication with zero matrix failed.");
}
