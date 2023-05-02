pub fn min_partitions(n: String) -> i32 {
    n.chars().map(|x| x as u8 - b'0').max().unwrap() as i32
}