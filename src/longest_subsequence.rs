pub fn longest_subsequence(a: Vec<i32>, diff: i32) -> i32 {
    let mut dp = std::collections::HashMap::<i32, i32>::new();
    let mut ans = 1;

    for i in a {
        let before = dp.get(&(i - diff)).cloned().unwrap_or(0);
        dp.insert(i, before + 1);
        ans = std::cmp::max(ans, dp[&i]);
    }

    return ans;
}
