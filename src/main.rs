#![allow(dead_code)]

use rand::prelude::*;
use petgraph::{matrix_graph::DiMatrix, dot::Dot, visit::IntoEdgeReferences};

use std::time::Instant;

#[derive(Debug)]
struct Graph {
    succ: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        let succ = vec![vec![]; size];
        Graph { succ }
    }

    fn size(&self) -> usize {
        self.succ.len()
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.succ[from].push(to)
    }
}

#[derive(Debug)]
struct Visit {
    node: usize,
    choice: usize,
}

#[derive(Debug)]
struct Traversals {
    graph: Graph,
    visits: Vec<Visit>,
    next: Vec<usize>,
    deps: Vec<isize>,
}

impl Traversals {
    fn new(graph: Graph) -> Self {
        let n = graph.size();

        let visits = Vec::with_capacity(graph.succ.len());
        let mut deps = vec![0; n];

        for ws in graph.succ.iter() {
            for &w in ws {
                deps[w] += 1;
            }
        }

        let mut next = vec![];

        for (v, &d) in deps.iter().enumerate() {
            if d == 0 {
                next.push(v);
            }
        }

        Traversals { graph, visits, next, deps }
    }

    fn is_done(&self) -> bool {
        self.visits.len() == self.graph.size()
    }

    fn get_order(&self) -> Vec<usize> {
        self.visits.iter().map(|v| v.node).collect()
    }

    fn advance(&mut self) {
        self.advance_to(self.next.len() - 1)
    }

    fn advance_to(&mut self, choice: usize) {
        let v = self.next.swap_remove(choice);

        self.visits.push(Visit { node: v, choice });

        for &w in self.graph.succ[v].iter() {
            self.deps[w] -= 1;
            if self.deps[w] == 0 {
                self.next.push(w);
            }
        }
    }

    fn backtrack(&mut self) -> usize {
        let Visit { node: v, choice } = self.visits.pop().unwrap();

        for &w in self.graph.succ[v].iter().rev() {
            if self.deps[w] == 0 {
                let z = self.next.pop().unwrap();
                assert_eq!(w, z);
            }
            self.deps[w] += 1;
        }

        // inverse of swap_remove
        self.next.push(v);
        let len = self.next.len();
        self.next.swap(choice, len - 1);

        choice
    }

    fn shift(&mut self) {
        let choice = self.backtrack();
        self.advance_to(choice - 1);
    }
}

impl Iterator for Traversals {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            while self.visits.last().map_or(false, |v| v.choice == 0) {
                self.backtrack();
            }
            if self.visits.is_empty() {
                return None
            }
            self.shift();
        }

        while !self.is_done() {
            self.advance();
        }

        Some(self.get_order())
    }
}

fn main() {
    let n = 12;
    let m = n * 2;

    let mut d = DiMatrix::<(), ()>::new();

    let seed = 233;// rand::thread_rng().gen();
    println!("# seed: {seed}");

    let mut rng = rand::rngs::StdRng::from_seed([seed; 32]);

    for _ in 0..n {
        d.add_node(());
    }

    for _ in 0..m {
        let v = rng.gen_range(0..(n-1));
        let w = rng.gen_range((v+1)..n);
        if !d.has_edge(v.into(), w.into()) {
            d.add_edge(v.into(), w.into(), ());
        }
    }

    {
        use petgraph::dot::Config;
        println!("{:?}", Dot::with_config(&d, &[Config::NodeNoLabel, Config::EdgeNoLabel]));
    }

    let mut g = Graph::new(n.into());

    for (v, w, ()) in d.edge_references() {
        g.add_edge(v.index(), w.index());
    }

    let now = Instant::now();

    let mut t = Traversals::new(g);

    for t0 in &mut t {
        println!("{t0:?}");
    }

    let elapsed = now.elapsed();
    eprintln!("{elapsed:.2?}")
}
