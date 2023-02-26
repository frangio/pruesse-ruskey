#![allow(dead_code, unused_imports)]

use std::time::Instant;

mod graph;
mod backtracking;
mod random;

use graph::Graph;
use backtracking::Traversals;
use random::random_graph;

fn main() {
    let n = 12;
    let m = n - 3;

    let g = random_graph(n, m, Some(20), false);

    let now = Instant::now();

    let mut t = Traversals::new(g);

    for t0 in &mut t {
        println!("{t0:?}");
    }

    let elapsed = now.elapsed();
    eprintln!("# {elapsed:.2?}")
}
