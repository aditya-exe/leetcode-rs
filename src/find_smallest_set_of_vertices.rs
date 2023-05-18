pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut exists = vec![false; n as usize];

    edges.iter().for_each(|x| exists[x[1] as usize] = true);

    let mut req = Vec::<i32>::new();

    for i in 0..n {
        if !exists[i as usize] {
            req.push(i);
        }
    }

    return req;
}