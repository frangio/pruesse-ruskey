#![allow(dead_code, unused_imports)]

mod glp;
mod nrpr;
mod graph;
mod backtracking;
mod random;

use glp::states;
use nrpr::NRPR;
use random::random_graph;

use std::time::Instant;

fn main() {
    let n = 20;
    let m = 3 * n;

    let g = random_graph(n, m, None, true);

    let now = Instant::now();

    let mut total = 0;

    for s in states::<NRPR>(g) {
        println!("{total}: {} {:?}", if s.s[0] { "+" } else { "-" }, s.l);
        total += 1;
    }

    println!("total={total}", total = total/2);

    // // traversals
    // let mut t = backtracking::Traversals::new(g);
    // for t0 in &mut t {
    //     println!("{t0:?}");
    // }

    let elapsed = now.elapsed();
    println!("{elapsed:.2?}")
}
