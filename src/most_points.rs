fn go(cur: usize, a: &Vec<Vec<i32>>, dp: &mut Vec<i64>) -> i64 {
    if cur>=a.len(){
        return 0;
    }

    if dp[cur]!=-1{
        return dp[cur];
    }

    let points = a[cur][0] as i64;
    let skip = a[cur][1];

    dp[cur] = std::cmp::max(points + go(cur + skip as usize + 1, a, dp), go(cur + 1, a, dp));

    return dp[cur];
}

pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut dp = vec![-1; questions.len()];

    return go(0, &questions, &mut dp);
}