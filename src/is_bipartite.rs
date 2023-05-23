use std::collections::HashMap;

fn dfs(v: i32, graph: &Vec<Vec<i32>>, color: &mut HashMap<i32, i32>)->bool {
    for u in &graph[v as usize] {
        if color.contains_key(u) {
            if color.get(u).unwrap() == color.get(&v).unwrap() {
                return false;
            }
        } else {
            color.insert(*u, 1 - *color.get(&v).unwrap());
            if !dfs(*u, graph, color) {
                return false;
            }
        }
    }
    return true;
}

pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let mut color = HashMap::new();

    for v in 0..graph.len(){
        color.entry(v as i32).or_insert(0);
        if !dfs(v as i32, &graph, &mut color) {
            return false;
        }
    }
    return true;
}