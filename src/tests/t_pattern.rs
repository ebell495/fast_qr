#[test]
fn position_pattern_03() {
    let mat = crate::default::create_matrix_from_version(3);
    // Top line
    assert!(mat[0][0]);
    assert!(mat[0][1]);
    assert!(mat[0][2]);
    assert!(mat[0][3]);
    assert!(mat[0][4]);
    assert!(mat[0][5]);
    assert!(mat[0][6]);

    // Left line
    assert!(mat[1][0]);
    assert!(mat[2][0]);
    assert!(mat[3][0]);
    assert!(mat[4][0]);
    assert!(mat[5][0]);
    assert!(mat[6][0]);

    // Right line
    assert!(mat[1][6]);
    assert!(mat[2][6]);
    assert!(mat[3][6]);
    assert!(mat[4][6]);
    assert!(mat[5][6]);
    assert!(mat[6][6]);

    // Bottom line
    assert!(mat[6][0]);
    assert!(mat[6][1]);
    assert!(mat[6][2]);
    assert!(mat[6][3]);
    assert!(mat[6][4]);
    assert!(mat[6][5]);
    assert!(mat[6][6]);

    // Inside square
    assert!(mat[2][2]);
    assert!(mat[2][3]);
    assert!(mat[2][4]);
    assert!(mat[3][2]);
    assert!(mat[3][3]);
    assert!(mat[3][4]);
    assert!(mat[4][2]);
    assert!(mat[4][3]);
    assert!(mat[4][4]);
}

/// Checks it's empty
#[test]
fn position_pattern_01() {
    let mat = crate::default::create_matrix_from_version(1);

    for i in 7..21 {
        for j in 7..21 {
            // Dark module
            if i == 13 && j == 8 {
                continue;
            }
            assert_eq!(mat[i][j], false);
        }
    }
}