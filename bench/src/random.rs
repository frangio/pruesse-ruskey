use topogen::graph::simple::SimpleGraph;
use rand::prelude::*;
use petgraph::{matrix_graph::DiMatrix, dot::{Dot, Config}, visit::IntoEdgeReferences, prelude::DiGraph, data::FromElements};

pub fn make_seed() -> usize {
    let seed = rand::thread_rng().gen();
    eprintln!("seed = {seed}");
    seed
}

pub fn make_rng(seed: Option<usize>) -> StdRng {
    let seed = seed.unwrap_or_else(make_seed);
    rand::rngs::StdRng::seed_from_u64(seed.try_into().unwrap())
}

pub fn random_graph(seed: Option<usize>, n: Option<usize>, m: Option<usize>, print: bool) -> SimpleGraph {
    let mut rng = make_rng(seed);

    let n = n.unwrap_or(rng.gen_range(10..55));

    let m_max = (n * (n-1)) / 2;
    let m = m.unwrap_or(rng.gen_range(m_max/2..m_max));

    let mut d = DiMatrix::<(), (), Option<()>, usize>::with_capacity(n);

    for _ in 0..n {
        d.add_node(());
    }

    let mut k = 0;
    while k < m {
        let v = rng.gen_range(0..(n-1));
        let w = rng.gen_range((v+1)..n);
        if !d.has_edge(v.into(), w.into()) {
            d.add_edge(v.into(), w.into(), ());
            k += 1;
        }
    }

    if print {
        println!("{:?}", Dot::with_config(&d, &[Config::NodeNoLabel, Config::EdgeNoLabel]));
    }

    let mut g = SimpleGraph::new(n.into());

    for (v, w, ()) in d.edge_references() {
        g.add_edge(v.index(), w.index());
    }

    g
}
