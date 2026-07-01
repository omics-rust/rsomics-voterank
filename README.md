# rsomics-voterank

VoteRank influential-node ranking of an undirected graph — a value-exact Rust
port of `networkx.voterank` (Zhang et al. 2016).

VoteRank ranks nodes by an iterative voting scheme: every node starts with
voting ability 1, each node's score is the sum of its neighbours' abilities, and
in each round the highest-scoring node is elected. The elected node's ability is
zeroed and each of its neighbours' abilities is reduced by `1 / average_degree`
(clamped at 0), weakening its region for subsequent rounds. The result is an
ordered list of influential seeds, most influential first.

## Usage

```sh
# edge list on stdin (u v per line; # comments and blank lines ignored)
rsomics-voterank < graph.edges

# top 10 seeds only
rsomics-voterank --number-of-nodes 10 graph.edges

# JSON array of labels
rsomics-voterank --json graph.edges
```

Input is an undirected edge list with string node labels. Parallel edges are
deduplicated and self-loops dropped (a simple `nx.Graph`). Node order — which
decides score ties — is first-appearance order in the edge list, matching
`nx.read_edgelist`. Output is one label per line in rank order (only
positively-voted nodes are returned).

## Origin

This crate is an independent Rust reimplementation of NetworkX's `voterank`
based on:

- The published method: Zhang, J.-X., Chen, D.-B., Dong, Q. & Zhao, Z.-D.
  (2016). *Identifying a set of influential spreaders in complex networks.*
  Scientific Reports 6, 27823. doi:10.1038/srep27823.
- The NetworkX 3.6.1 implementation
  (`networkx.algorithms.centrality.voterank_alg`), which is BSD-3-Clause and may
  be read and cited.
- Black-box behaviour testing against `nx.voterank` (value-exact ordered-list
  equality on hand graphs, the karate club, a score-tie graph, and `gnm`
  random graphs).

Matched semantics: `average_degree = 2m/n` computed once; per-round score is the
sum of neighbours' voting abilities; the highest-scoring node is elected with
ties broken by first-appearance (node iteration) order; the elected node's
ability is set to 0 and each neighbour's ability is decremented by
`1/average_degree` then clamped to `max(0, ability)`; ranking stops after
`number_of_nodes` elections (capped to `|G|`) or as soon as the top score is 0.

License: MIT OR Apache-2.0.
Upstream credit: [NetworkX](https://networkx.org/) (BSD-3-Clause).
