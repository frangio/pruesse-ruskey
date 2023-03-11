#![allow(dead_code, unused_imports)]

mod graph;
mod backtracking;
mod random;

use std::time::Instant;

use graph::Graph;
use random::random_graph;

use bit_vec::BitVec;

#[derive(Debug)]
enum Move {
    None,
    Swap(usize, usize),
    FlipSign,
}

fn sorted<T: Ord>(a: T, b: T) -> (T, T) {
    if b < a {
        (b, a)
    } else {
        (a, b)
    }
}

fn next_move(has_edge: impl Fn(usize, usize) -> bool, e: bool, s: bool, l: &[usize], i: usize, j: usize) -> Move {
    if j == 1 {
        if s == !e {
            Move::None
        } else if j + 1 >= l.len() || has_edge(l[j], l[j + 1]) {
            Move::FlipSign
        } else {
            Move::Swap(j, j + 1)
        }
    } else if i == 0 && s == !e {
        Move::Swap(j, j - 1)
    } else {
        let rw = (j % 2 == 1) != e;
        let u = match rw == s {
            true if i + 1 != j && !has_edge(l[i], l[i + 1]) => Some(i + 1),
            false if i > 1 || (i > 0 && s == e) => Some(i - 1),
            _ => None,
        };
        // println!("s={s} rw={rw} u={u:?} j={j}");
        if let Some(u) = u {
            Move::Swap(i, u)
        } else if rw == s {
            Move::FlipSign
        } else if j + 1 < l.len() && !has_edge(l[j], l[j + 1]) {
            Move::Swap(j, j + 1)
        } else if rw == e {
            Move::Swap(i, i - 1)
        } else {
            Move::FlipSign
        }
    }
}

fn edge_pos(n: usize, v: usize, w: usize) -> usize {
    n * v + w
}

fn nrpr(g: &Graph) {
    let n = g.size();

    // preconditioning

    let mut adj = BitVec::from_elem(n * n, false);

    let mut l0 = Vec::with_capacity(n);
    let mut j = vec![];
    let mut min = vec![];
    let mut in_deg = vec![0; n];

    for (v, w) in g.edges() {
        adj.set(edge_pos(n, v, w), true);
        in_deg[w] += 1;
    }

    for (v, &d) in in_deg.iter().enumerate() {
        if d == 0 {
            min.push(v);
        }
    }

    while let Some(a) = min.pop() {
        l0.push(a);

        let b = min.pop();

        if let Some(b) = b {
            let i = l0.len();
            j.push(i - 1);
            j.push(i);
            l0.push(b);
        }

        for v in [Some(a), b].into_iter().flatten() {
            for w in g.successors(v) {
                in_deg[w] -= 1;
                if in_deg[w] == 0 {
                    min.push(w);
                }
            }
        }
    }

    let has_edge = move |v: usize, w: usize| adj[edge_pos(n, v, w)];

    let k = j.len() / 2;
    let mut l = l0;
    let mut ix = j.clone();
    let mut s = vec![true; k + 1];
    let mut e = vec![true; k];
    let mut v = vec![true; k + 1];

    println!("{} {l:?}", if s[0] { "+" } else { "-" });

    let mut total = 1;

    while let Some(i) = v.iter().position(|&v| v) {
        // println!("i={i} s={s:?} v={v:?} e={e:?}");

        if i == k {
            if k > 0 {
                let p = 2 * k - 2;
                l.swap(ix[p], ix[p + 1]);
                ix.swap(p, p + 1);
            }
            s[k] = false;
            v[k] = false;
        } else {
            let ji = j[2 * i];
            let (i1, i2) = sorted(ix[2 * i], ix[2 * i + 1]);
            let m = next_move(&has_edge, e[i], s[i], &l[ji..], i1 - ji, i2 - ji);
            match m {
                Move::Swap(a, b) => {
                    let a = ji + a;
                    let b = ji + b;
                    l.swap(a, b);
                    if a == ix[2 * i] {
                        ix[2 * i] = b;
                    }
                    if a == ix[2 * i + 1] {
                        ix[2 * i + 1] = b;
                    }
                },
                Move::FlipSign => {
                    s[i] = !s[i];
                    if i > 0 {
                        let p = 2 * i - 2;
                        l.swap(ix[p], ix[p + 1]);
                        ix.swap(p, p + 1);
                    }
                },
                Move::None => {
                    break
                }
            }

            if (ix[2 * i], ix[2 * i + 1]) == (ji, ji + 1) && s[i] != e[i] {
                v[i] = false;
                e[i] = !e[i];
            } else {
                v[i] = true;
            }
        }

        for p in 0..i {
            v[p] = true;
        }

        println!("{} {l:?}", if s[0] { "+" } else { "-" });

        total += 1;
    }

    println!("total={total}");
}

fn main() {
    let n = 4;
    let m = 0;

    let g = random_graph(n, m, None, true);

    let now = Instant::now();

    nrpr(&g);

    // // traversals
    // let mut t = backtracking::Traversals::new(g);
    // for t0 in &mut t {
    //     println!("{t0:?}");
    // }

    let elapsed = now.elapsed();
    println!("{elapsed:.2?}")
}
