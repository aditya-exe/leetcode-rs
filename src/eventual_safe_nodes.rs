pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::new();

    let mut stack = vec![false; graph.len()];
    for i in 0..graph.len() {
        let mut visited = vec![false; graph.len()];
        if is_safe(i, &graph, &mut visited, &mut stack) {
            ans.push(i as i32);
        }
    }

    ans
}

fn is_safe(
    i: usize,
    graph: &Vec<Vec<i32>>,
    visited: &mut Vec<bool>,
    stack: &mut Vec<bool>,
) -> bool {
    if !visited[i] {
        visited[i] = true;
        stack[i] = true;

        for &j in &graph[i] {
            if !visited[j as usize] && !is_safe(j as usize, graph, visited, stack) {
                return false;
            } else if stack[j as usize] {
                return false;
            }
        }
    }
    stack[i] = false;
    return true;
}