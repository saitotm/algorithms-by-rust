use std::collections::BinaryHeap;

struct Edge {
    to: usize,
    cost: i64,
}

type Graph = Vec<Vec<Edge>>;
const INF: i64 = 1 << 60;

fn dijkstra(g: &Graph, s: usize) -> Vec<i64> {
    let mut dis = vec![INF; g.len()];
    let mut bheap = BinaryHeap::new();

    dis[s] = 0;
    bheap.push((dis[s], s));
    while let Some(peek) = bheap.pop() {
        let v = peek.1; 
        if dis[v] < peek.0 {
            continue;
        }

        for e in g[v].iter() {
            if dis[e.to] > dis[v] + e.cost {
                dis[e.to] = dis[v] + e.cost;
                bheap.push((dis[e.to], e.to));
            }
        }
    }

    dis
}
