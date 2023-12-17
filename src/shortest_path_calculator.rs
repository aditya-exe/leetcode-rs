use std::collections::{BinaryHeap, HashMap, HashSet};

struct Graph {
    graph: HashMap<usize, Vec<(i32, usize)>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut graph = Self {
            graph: HashMap::new(),
        };
        edges.into_iter().for_each(|edge| graph.add_edge(edge));
        return graph;
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let (from, to, cost) = (edge[0] as usize, edge[1] as usize, -edge[2]);
        self.graph
            .entry(from)
            .and_modify(|v| v.push((cost, to)))
            .or_insert(vec![(cost, to)]);
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let node1 = node1 as usize;
        let node2 = node2 as usize;

        let mut costs = HashMap::<usize, i32>::new();
        let mut seen = HashSet::<usize>::new();
        let mut heap = BinaryHeap::<(i32, usize)>::new();

        heap.push((0, node1));

        while let Some((cost, node)) = heap.pop() {
            if node == node2 {
                return -cost;
            }

            costs
                .entry(node)
                .and_modify(|v| *v = cost.min(*v))
                .or_insert(cost);

            if seen.contains(&node) {
                continue;
            }

            seen.insert(node);

            if let Some(neighbours) = self.graph.get(&node) {
                for (n_cost, neighbour) in neighbours {
                    let cost = cost + *n_cost;
                    heap.push((cost, *neighbour));
                }
            }
        }

        return -1;
    }
}
