pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let n = mat.len();

    for i in 0..n {
        res += mat[i][i];
        res += mat[n - i - 1][i];
    }

    if n % 2 != 0 {
        res -= mat[n / 2][n / 2];
    }

    return res;
}