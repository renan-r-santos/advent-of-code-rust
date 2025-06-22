//! LAN Party

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use petgraph::algo::maximal_cliques::maximal_cliques;
use petgraph::graph::{NodeIndex, UnGraph};

type Input<'a> = UnGraph<&'a str, (), u32>;

pub fn parse(input: &str) -> Input {
    let mut graph = UnGraph::new_undirected();
    let mut node_map = HashMap::new();

    for line in input.lines() {
        let a = &line[0..2];
        let b = &line[3..5];

        let node_a = *node_map.entry(a).or_insert_with(|| graph.add_node(a));
        let node_b = *node_map.entry(b).or_insert_with(|| graph.add_node(b));

        graph.add_edge(node_a, node_b, ());
    }

    graph
}

pub fn part1(g: &Input) -> i64 {
    // Based on https://networkx.org/documentation/stable/_modules/networkx/algorithms/cluster.html#triangles
    let mut later_nbrs = HashMap::new();

    for node in g.node_indices() {
        let mut neighbors = HashSet::new();
        for neighbor in g.neighbors(node) {
            if !later_nbrs.contains_key(&neighbor) {
                neighbors.insert(neighbor);
            }
        }
        later_nbrs.insert(node, neighbors);
    }

    let mut count = 0;

    for (node1, neighbors) in &later_nbrs {
        for node2 in neighbors {
            for node3 in neighbors.intersection(&later_nbrs[node2]) {
                if starts_with_t(g, node1) || starts_with_t(g, node2) || starts_with_t(g, node3) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part2(g: &Input) -> String {
    let cliques = maximal_cliques(g);
    let max_cliques = cliques.iter().max_by_key(|clique| clique.len()).unwrap();
    max_cliques.iter().map(|node| *g.node_weight(*node).unwrap()).sorted().join(",")
}

fn starts_with_t(g: &Input, node: &NodeIndex) -> bool {
    g.node_weight(*node).unwrap().starts_with('t')
}
