use crate::graph::Graph;
use rand::prelude::*;
use petgraph::{matrix_graph::DiMatrix, dot::{Dot, Config}, visit::IntoEdgeReferences};

pub fn random_graph(n: usize, m: usize, seed: Option<usize>, print: bool) -> Graph {
    let mut d = DiMatrix::<(), (), Option<()>, usize>::with_capacity(n);

    for _ in 0..n {
        d.add_node(());
    }

    let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());

    eprintln!("# seed: {seed}");

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed.try_into().unwrap());

    let mut k = 0;
    while k <= m {
        let v = rng.gen_range(0..(n-1));
        let w = rng.gen_range((v+1)..n);
        if !d.has_edge(v.into(), w.into()) {
            d.add_edge(v.into(), w.into(), ());
        }
        k += 1;
    }

    if print {
        eprintln!("{:?}", Dot::with_config(&d, &[Config::NodeNoLabel, Config::EdgeNoLabel]));
    }

    let mut g = Graph::new(n.into());

    for (v, w, ()) in d.edge_references() {
        g.add_edge(v.index(), w.index());
    }

    g
}
