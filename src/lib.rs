//! VoteRank influential-node ranking — value-exact port of
//! `networkx.voterank` (Zhang et al. 2016).
//!
//! Every node starts with voting ability 1. Each round the score of a node is
//! the sum of its neighbours' voting abilities; the highest-scoring node is
//! elected, its ability zeroed, and each of its neighbours' abilities reduced
//! by `1 / avg_degree` (clamped at 0). `avg_degree` is `2m/n`, computed once.
//! Ranking stops after `number_of_nodes` elections or as soon as the top score
//! is 0 (only positively-voted nodes are returned).
//!
//! Ties in the top score resolve to the node appearing first in NetworkX's node
//! iteration order, which for edge-list input is first-appearance order. Labels
//! are interned to `0..n` in that order so the integer index carries the
//! tie-break, and the hot loop never touches a hash map.
//!
//! Zhang, J.-X. et al. (2016). Identifying a set of influential spreaders in
//! complex networks. Sci. Rep. 6, 27823. doi:10.1038/srep27823.

use std::collections::HashMap;

/// Undirected simple graph over interned integer node ids, in first-appearance
/// order.
pub struct Graph {
    idx_to_node: Vec<String>,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn intern(&mut self, name: &str, table: &mut HashMap<String, usize>) -> usize {
        if let Some(&idx) = table.get(name) {
            return idx;
        }
        let idx = self.idx_to_node.len();
        table.insert(name.to_owned(), idx);
        self.idx_to_node.push(name.to_owned());
        self.adj.push(Vec::new());
        idx
    }

    pub fn len(&self) -> usize {
        self.idx_to_node.len()
    }

    pub fn is_empty(&self) -> bool {
        self.idx_to_node.is_empty()
    }
}

/// Parse a whitespace-delimited `u v` edge list. `#` comments and blank lines
/// are skipped, parallel edges deduplicated, and self-loops dropped — the
/// simple undirected graph `nx.read_edgelist` builds, with node insertion order
/// equal to first appearance in the file.
pub fn parse_edge_list(input: &str) -> Graph {
    let mut g = Graph {
        idx_to_node: Vec::new(),
        adj: Vec::new(),
    };
    let mut table = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let (Some(u), Some(v)) = (parts.next(), parts.next()) else {
            continue;
        };
        let ui = g.intern(u, &mut table);
        let vi = g.intern(v, &mut table);
        if ui == vi {
            continue;
        }
        if !g.adj[ui].contains(&vi) {
            g.adj[ui].push(vi);
            g.adj[vi].push(ui);
        }
    }
    g
}

/// Ordered interned node ids of the elected seeds.
pub fn voterank(g: &Graph, number_of_nodes: Option<usize>) -> Vec<usize> {
    let n = g.len();
    if n == 0 {
        return Vec::new();
    }
    let rounds = match number_of_nodes {
        Some(k) if k <= n => k,
        _ => n,
    };

    let total_degree: usize = g.adj.iter().map(Vec::len).sum();
    let avg_degree = total_degree as f64 / n as f64;
    let decrement = 1.0 / avg_degree;

    let mut ability = vec![1.0f64; n];
    let mut score = vec![0.0f64; n];
    let mut seeds = Vec::with_capacity(rounds);

    // Votes accumulate in `G.edges()` order, each edge adding to both endpoints.
    // Float summation is order-sensitive, so matching this exact order (not a
    // per-node sum) is what keeps near-ties — and thus the ranking — value-exact.
    let mut edges_ordered: Vec<(usize, usize)> = Vec::with_capacity(total_degree / 2);
    for u in 0..n {
        for &v in &g.adj[u] {
            if v > u {
                edges_ordered.push((u, v));
            }
        }
    }

    for _ in 0..rounds {
        score.iter_mut().for_each(|s| *s = 0.0);
        for &(u, v) in &edges_ordered {
            score[u] += ability[v];
            score[v] += ability[u];
        }
        for &e in &seeds {
            score[e] = 0.0;
        }

        // First index attaining the max score — first-appearance tie-break.
        let mut best = 0usize;
        for u in 1..n {
            if score[u] > score[best] {
                best = u;
            }
        }
        if score[best] == 0.0 {
            break;
        }

        seeds.push(best);
        ability[best] = 0.0;
        for &v in &g.adj[best] {
            ability[v] = (ability[v] - decrement).max(0.0);
        }
    }
    seeds
}

/// End-to-end: parse the edge list and return the ranked node labels.
pub fn voterank_from_edge_list(input: &str, number_of_nodes: Option<usize>) -> Vec<String> {
    let g = parse_edge_list(input);
    voterank(&g, number_of_nodes)
        .into_iter()
        .map(|i| g.idx_to_node[i].clone())
        .collect()
}
