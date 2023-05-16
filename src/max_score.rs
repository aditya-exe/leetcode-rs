pub fn max_score(nums: Vec<i32>) -> i32 {
    let n = 1 << nums.len();
    let mut dp = vec![-1; n];

    return go(0, 0, &nums, &mut dp);
}

fn go(mask: usize, pairs_picked: usize, nums: &Vec<i32>, dp: &mut Vec<i32>) -> i32 {
    if 2 * pairs_picked == nums.len() {
        return 0;
    }

    if dp[mask] != -1 {
        return dp[mask];
    }

    let mut max_score = 0;

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if ((mask >> i) & 1) == 1 || ((mask >> j) & 1) == 1 {
                continue;
            }
            let new_mask = mask | (1 << i) | (1 << j);
            let cur_score = (pairs_picked + 1) * gcd(nums[i], nums[j]) as usize;
            let reamaining_score = go(new_mask, pairs_picked + 1, nums, dp);

            max_score = std::cmp::max(max_score, cur_score + reamaining_score as usize);
        }
    }

    dp[mask] = max_score as i32;
    return dp[mask];
}

fn gcd(a: i32, b: i32) -> i32 {
    if a < b { return gcd(b, a); }
    if a % b == 0 { return b; }
    gcd(b, a % b)
}