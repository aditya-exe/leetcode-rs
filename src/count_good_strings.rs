const MOD: i32 = 1_000_000_007;

pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    let mut dp = vec![-1; high as usize + 1];
    dp[0] = 1;

    let mut ans = 0;
    for end in low as usize..=high as usize {
        ans += go(end, zero as usize, one as usize, &mut dp);
        ans %= MOD;
    }
    return ans;
}

fn go(end: usize, zero: usize, one: usize, dp: &mut Vec<i32>) -> i32 {
    if dp[end] != -1 {
        return dp[end];
    }

    let mut cnt = 0;
    if end >= one {
        cnt += go(end - one, zero, one, dp);
    }
    if end >= zero {
        cnt += go(end - zero, zero, one, dp);
    }
    dp[end] = cnt % MOD;
    return dp[end];
}