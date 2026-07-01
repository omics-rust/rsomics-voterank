use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use rsomics_voterank::{parse_edge_list, voterank};

const GNM: &str = include_str!("../tests/golden/gnm_300_1500_s1.txt");

fn bench(c: &mut Criterion) {
    // Parse once; time the ranking only, matching the networkx oracle which
    // pre-builds the graph and times nx.voterank.
    let g = parse_edge_list(GNM);
    c.bench_function("voterank_gnm_300_1500", |b| {
        b.iter(|| black_box(voterank(black_box(&g), None)));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
