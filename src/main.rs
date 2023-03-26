#![allow(dead_code, unused_imports, unused_variables)]

mod glp;
mod nrpr;
mod graph;
mod backtracking;
mod random;

use glp::{states, deltas, GLPLoop, GLPLoopFree};
use graph::Graph;
use nrpr::NRPR;
use rand::Rng;
use random::{random_graph, make_rng};

use std::{time::Instant, sync::{Mutex, Arc}};

fn count_backtracking(g: Graph) -> usize {
    let mut total = 0;
    let mut t = backtracking::Traversals::new(g);
    for _ in &mut t {
        total += 1;
    }
    total
}

fn count_nrpr_loop_free(g: Graph) -> usize {
    let mut total = 0;
    for _ in deltas::<NRPR, GLPLoopFree>(g) {
        total += 1;
    }
    total / 2
}

fn main() {
    let mut rng = make_rng(None);

    let pool = threadpool::Builder::new().build();

    for i in 0..250 {
        let seed = rng.gen();

        pool.execute(move || {
            let g = random_graph(Some(seed), None, None, false);

            let n = g.size();
            let m = g.edge_count();

            let mut backtracking_elapsed = 0.0;
            let mut loop_free_elapsed = 0.0;
            let mut total = 0;

            let v = 2;

            for j in 0..v {
                match (i + j) % v {
                    0 => {
                        let now = Instant::now();
                        total = count_nrpr_loop_free(g.clone());
                        loop_free_elapsed = now.elapsed().as_secs_f64();
                    },

                    1 => {
                        let now = Instant::now();
                        total = count_backtracking(g.clone());
                        backtracking_elapsed = now.elapsed().as_secs_f64();
                    }

                    _ => unreachable!(),
                }
            }

            println!("{{\"backtracking\":{backtracking_elapsed:.?},\"loop_free\":{loop_free_elapsed:.?},\"n\":{n},\"m\":{m},\"total\":{total},\"seed\":{seed}}}");
        })
    }

    pool.join()
}
