pub fn max_score(s: String) -> i32 {
    s[..s.len() - 1]
        .chars()
        .fold(
            (0, s.chars().filter(|&c| c == '1').count(), 0),
            |(x, y, ans), c| match c {
                '1' => (x, y - 1, std::cmp::max(ans, x + y - 1)),
                '0' => (x + 1, y, std::cmp::max(ans, x + y + 1)),
                _ => (x, y, ans),
            },
        )
        .2 as i32
}
