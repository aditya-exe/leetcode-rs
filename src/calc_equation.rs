use std::collections::{HashMap, HashSet};

pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let mut graph = HashMap::<String, HashMap<String, f64>>::new();

    equations.iter().enumerate().for_each(|(i, x)| {
        graph.entry(x[0].clone()).or_default().insert(x[1].clone(), values[i]);
        graph.entry(x[1].clone()).or_default().insert(x[0].clone(), 1.0 / values[i]);
    });

    let mut ans = vec![];
    let mut visited = HashSet::<String>::new();

    queries.iter().for_each(|q| {
        let res = dfs(q[0].clone(), &q[1], 1.0, &graph, &mut visited);
        match res {
            Some(result) => ans.push(result),
            None => ans.push(-1.0)
        };
        visited.clear();
    });

    return ans;
}

fn dfs(start: String, goal: &String, value: f64, graph: &HashMap<String, HashMap<String, f64>>, visited: &mut HashSet<String>) -> Option<f64> {
    if !graph.contains_key(&start) {
        return None;
    }
    if start.eq(goal) {
        return Some(value);
    }
    visited.insert(start.clone());

    for (neighbor, neighbor_value) in graph[&start].iter() {
        if visited.contains(neighbor) {
            continue;
        }

        let result = dfs(neighbor.clone(), goal, value * neighbor_value, graph, visited);
        match result {
            Some(result) => {
                return Some(result);
            }
            None => {
                visited.remove(neighbor);
            }
        }
    }

    return None;
}