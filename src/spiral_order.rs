pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let h = matrix.len();
    let w = matrix[0].len();
    let mut ans = Vec::with_capacity(h * w);
    let mut i = 0;
    let mut j = 0;
    let mut dir = 1usize;
    let mut iter = std::iter::once(w - 1)
        .chain((1..w).rev()).zip((0..h).rev());
    for (x, y) in iter {
        for _ in 0..x {
            ans.push(matrix[i][j]);
            j = j.wrapping_add(dir);
        }
        for _ in 0..y {
            ans.push(matrix[i][j]);
            i = i.wrapping_add(dir);
        }
        dir = 0usize.wrapping_sub(dir);
    }
    ans.push(matrix[i][j]);
    ans
}