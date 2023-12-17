
pub fn min_add_to_make_valid(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let mut balance = 0;
    let mut ans = 0;

    s.chars().for_each(|c| {
        match c {
            '(' => balance += 1,
            ')' => balance -= 1,
            _ => unreachable!(),
        }
        if balance == -1 {
            ans += 1;
            balance += 1;
        }
    });

    return ans + balance;
}
