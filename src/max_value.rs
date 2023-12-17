pub fn max_value(mut a: Vec<Vec<i32>>, k: i32) -> i32 {
    a.sort_unstable();
    let next: Vec<usize> = a
        .iter()
        .map(|v| a.partition_point(|v2| v2[0] <= v[1]))
        .collect();

    let mut dp = vec![vec![0; k as usize + 1]; a.len() + 1];

    for j in (0..k as usize).rev() {
        for i in (0..a.len()).rev() {
            dp[i][j] = std::cmp::max(dp[i + 1][j], a[i][2] + dp[next[i]][j + 1]);
        }
    }

    return dp[0][0];
}
