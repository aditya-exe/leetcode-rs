pub fn can_finish(n: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut adj = std::collections::HashMap::<i32, Vec<i32>>::new();
    prerequisites.into_iter().for_each(|edge| {
        let u = edge[0];
        let v = edge[1];
        adj.entry(v).or_default().push(u);
    });
    let mut vis = vec![0; n as usize];

    for i in 0..n {
        match is_cycle(&adj, &mut vis, i) {
            true => return false,
            false => (),
        }
    }
    return true;
}

fn is_cycle(adj: &std::collections::HashMap<i32, Vec<i32>>, vis: &mut Vec<i32>, i: i32) -> bool {
    if vis[i as usize] == 1 {
        return true;
    }
    if vis[i as usize] == 0 {
        vis[i as usize] = 1;

        match adj.get(&i) {
            Some(edges) => {
                for &edge in edges {
                    match is_cycle(adj, vis, edge) {
                        true => return true,
                        false => (),
                    }
                }
            }
            _ => (),
        }
    }
    vis[i as usize] = 2;
    return false;
}
