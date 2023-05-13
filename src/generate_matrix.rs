pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    fn pivot(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (h, w) = (matrix.len(), matrix[0].len());
        let mut new_matrix = vec![vec![0; h]; w];

        for i in 0..h {
            for j in 0..w {
                new_matrix[j][i] = matrix[h - i - 1][j];
            }
        }

        new_matrix
    }

    fn step(mat: Vec<Vec<i32>>, mut lo: &mut i32) -> Vec<Vec<i32>> {
        let hi = *lo;
        *lo -= mat.len() as i32;

        let mut pivoted_matrix = pivot(mat);
        let mut matrix = vec![
            (*lo..hi).collect::<Vec<i32>>()
        ];
        matrix.append(&mut pivoted_matrix);

        matrix
    }

    let mut res = vec![vec![n * n]];
    let mut l = n * n;

    while l > 1 {
        res = step(res, &mut l);
    }

    return res;
}