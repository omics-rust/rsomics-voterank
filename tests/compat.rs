//! Value-exact compatibility with networkx 3.6.1 `nx.voterank`.
//!
//! Expected sequences were generated once with networkx 3.6.1 by feeding the
//! same golden edge-list files through `nx.read_edgelist(..., nodetype=str)` so
//! node iteration order — which drives the tie-break — matches this crate's
//! first-appearance interning. They are frozen here as constants; no Python or
//! subprocess runs at test time.

use rsomics_voterank::voterank_from_edge_list;

const KARATE: &str = include_str!("golden/karate.txt");
const TIE: &str = include_str!("golden/tie.txt");
const GNM_A: &str = include_str!("golden/gnm_a.txt");
const GNM_B: &str = include_str!("golden/gnm_b.txt");
const BA_250_4: &str = include_str!("golden/ba_250_4.txt");
const SELFLOOP_TRIANGLE: &str = include_str!("golden/selfloop_triangle.txt");
const SELFLOOP_SINGLE: &str = include_str!("golden/selfloop_single.txt");
const SELFLOOP_STAR: &str = include_str!("golden/selfloop_star.txt");

// A scale-free graph exercises the many near-tie score comparisons that only
// agree with networkx when votes accumulate in G.edges() order (not per node).
const BA_250_4_EXPECTED: &[&str] = &[
    "11", "0", "9", "10", "6", "5", "13", "2", "36", "52", "3", "12", "55", "19", "33", "4", "7",
    "30", "16", "14", "31", "65", "8", "15", "23", "18", "27", "73", "62", "21", "125", "46", "41",
    "51", "57", "104", "29", "42", "161", "26", "78", "22", "39", "100", "183", "17", "69", "76",
    "40", "32", "48", "37", "90", "47", "75", "66", "179", "53", "82", "227", "134", "172", "58",
    "20", "63", "119", "64", "158", "141", "67", "59", "86", "80", "72", "147", "205", "214", "68",
    "44", "56", "178", "54", "121", "152", "1", "137", "199", "99", "34", "28", "111", "115",
    "131", "174", "106", "117", "60", "166", "116", "188", "206", "142", "43", "38", "94", "112",
    "89", "110", "84", "102", "70", "160", "91", "120", "79", "135", "109", "25", "118", "49",
    "24", "45", "124", "150", "98", "122", "143", "148", "192", "71", "130", "191", "126", "92",
    "151", "168", "154", "127", "85", "93", "230", "182", "35", "105", "176", "181", "164", "140",
    "103",
];

fn assert_seq(got: Vec<String>, expected: &[&str]) {
    let expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
    assert_eq!(got, expected);
}

#[test]
fn hand_graph_docstring() {
    // nx.voterank(nx.Graph([(0,1),(0,2),(0,3),(1,4)])) == [0, 1]
    let edges = "0 1\n0 2\n0 3\n1 4\n";
    assert_seq(voterank_from_edge_list(edges, None), &["0", "1"]);
}

#[test]
fn ba_250_4_full() {
    assert_seq(voterank_from_edge_list(BA_250_4, None), BA_250_4_EXPECTED);
}

#[test]
fn karate_club() {
    assert_seq(
        voterank_from_edge_list(KARATE, None),
        &[
            "33", "0", "32", "2", "1", "5", "31", "23", "6", "3", "24", "29", "4", "8",
        ],
    );
}

#[test]
fn score_tie_breaks_on_first_appearance() {
    // Two identical degree-3 stars centred on `a` and `e`; both score 3 in round
    // one. networkx elects `a` because it appears first in node iteration order.
    assert_seq(voterank_from_edge_list(TIE, None), &["a", "e"]);
}

#[test]
fn gnm_a_full() {
    assert_seq(
        voterank_from_edge_list(GNM_A, None),
        &[
            "14", "34", "17", "46", "40", "4", "15", "23", "27", "35", "13", "38", "41", "7", "20",
            "12", "29", "47", "42", "32", "28", "16", "48", "3", "1", "5", "45", "36", "49", "0",
            "25", "6", "10",
        ],
    );
}

#[test]
fn gnm_b_full() {
    assert_seq(
        voterank_from_edge_list(GNM_B, None),
        &[
            "19", "53", "7", "50", "78", "71", "15", "46", "43", "28", "9", "8", "57", "13", "25",
            "11", "59", "64", "44", "16", "55", "33", "10", "26", "3", "23", "61", "18", "66",
            "30", "77", "40", "17", "35", "31", "62", "20", "12", "29", "36", "45", "73", "27",
            "60", "63", "67", "0", "6", "41", "65",
        ],
    );
}

#[test]
fn gnm_b_partial_five() {
    assert_seq(
        voterank_from_edge_list(GNM_B, Some(5)),
        &["19", "53", "7", "50", "78"],
    );
}

#[test]
fn determinism() {
    let a = voterank_from_edge_list(GNM_A, None);
    let b = voterank_from_edge_list(GNM_A, None);
    assert_eq!(a, b);
}

// networkx keeps self-loops: a self-looped node has degree 2 and votes for
// itself, so it can be elected. Expected sequences regenerated from
// networkx 3.6.1 `nx.voterank`.
#[test]
fn self_loop_node_is_elected() {
    // nx.voterank(nx.Graph([('a','b'),('b','c'),('c','a'),('d','d')]))
    //   == ['a', 'd', 'b']  — the self-looped d outranks b/c in round two.
    assert_seq(
        voterank_from_edge_list(SELFLOOP_TRIANGLE, None),
        &["a", "d", "b"],
    );
}

#[test]
fn lone_self_loop_is_elected() {
    // A single self-looped node has degree 2 and votes for itself.
    assert_seq(voterank_from_edge_list(SELFLOOP_SINGLE, None), &["d"]);
}

#[test]
fn self_loop_and_star() {
    assert_seq(voterank_from_edge_list(SELFLOOP_STAR, None), &["a", "e"]);
}

#[test]
fn empty_graph_is_empty() {
    assert!(voterank_from_edge_list("", None).is_empty());
    assert!(voterank_from_edge_list("# just a comment\n\n", None).is_empty());
}

#[test]
fn number_of_nodes_over_len_returns_full() {
    // number_of_nodes larger than |G| is capped to the full ranking.
    let full = voterank_from_edge_list(KARATE, None);
    let capped = voterank_from_edge_list(KARATE, Some(1000));
    assert_eq!(full, capped);
}
