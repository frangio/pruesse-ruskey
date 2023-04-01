use crate::glp::{GLPSubProc, states};
use crate::graph::Graph;

use bit_vec::BitVec;

#[derive(Debug)]
pub enum Move {
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
        if let Some(u) = u {
            Move::Swap(i, u)
        } else if rw == s && i > 0 {
            Move::FlipSign
        } else if j + 1 < l.len() && !has_edge(l[j], l[j + 1]) {
            Move::Swap(j, j + 1)
        } else if rw == e && i > 0 {
            Move::Swap(i, i - 1)
        } else {
            Move::FlipSign
        }
    }
}

fn edge_pos(n: usize, v: usize, w: usize) -> usize {
    n * v + w
}

pub struct NRPR {
    n: usize,
    adj: BitVec,
    k: usize,
    pub l: Vec<usize>,
    j: Vec<usize>,
    ix: Vec<usize>,
    pub s: Vec<bool>,
    e: Vec<bool>,
}

impl NRPR {
    fn new(g: Graph) -> Self {
        let n = g.size();

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

        let k = j.len() / 2;
        let l = l0;
        let ix = j.clone();
        let s = vec![true; k + 1];
        let e = vec![true; k];

        NRPR { n, adj, k, l, j, ix, s, e }
    }
}

impl GLPSubProc for NRPR {
    type Delta = Move;

    fn size(&self) -> usize {
        self.k + 1
    }

    fn execute(&mut self, i: usize) -> (bool, Self::Delta) {
        let &mut NRPR {
            n,
            k,
            ref adj,
            ref mut l,
            ref mut j,
            ref mut ix,
            ref mut s,
            ref mut e,
        } = self;

        if i == k {
            let m = if k > 0 {
                let p = 2 * k - 2;
                l.swap(ix[p], ix[p + 1]);
                ix.swap(p, p + 1);
                Move::Swap(ix[p], ix[p + 1])
            } else {
                Move::None
            };
            s[k] = false;
            (false, m)
        } else {
            let ji = j[2 * i];
            let (i1, i2) = sorted(ix[2 * i], ix[2 * i + 1]);

            let m = next_move(|v, w| adj[edge_pos(n, v, w)], e[i], s[i], &l[ji..], i1 - ji, i2 - ji);

            let m = match m {
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
                    Move::Swap(a, b)
                },
                Move::FlipSign => {
                    s[i] = !s[i];
                    if i > 0 {
                        let p = 2 * i - 2;
                        l.swap(ix[p], ix[p + 1]);
                        ix.swap(p, p + 1);
                        Move::Swap(ix[p], ix[p + 1])
                    } else {
                        Move::FlipSign
                    }
                },
                Move::None => {
                    unreachable!();
                }
            };

            let (i1, i2) = sorted(ix[2 * i], ix[2 * i + 1]);

            let vi = if (i1, i2) == (ji, ji + 1) && s[i] != e[i] {
                e[i] = !e[i];
                false
            } else {
                true
            };

            (vi, m)
        }
    }
}

pub fn toposorts(g: Graph) -> impl Iterator<Item = Vec<usize>> {
    states(NRPR::new(g)).filter_map(|s| if s.s[0] { Some(s.l.clone()) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::HashSet;

    fn factorial(n: usize) -> usize {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    }

    fn binomial(n: usize, k: usize) -> usize {
        if k > n {
            return 0;
        }
        let k = k.min(n - k);
        let mut result = 1;
        for i in 0..k {
            result *= n - i;
            result /= i + 1;
        }
        result
    }

    #[test]
    fn test_0_edges() {
        for n in 2..7 {
            let g = Graph::new(n);
            let ts = toposorts(g).collect::<HashSet<_>>();
            assert_eq!(ts.len(), factorial(n))
        }
    }

    fn gen_graph_1_edge(min: usize, max: usize) -> impl Strategy<Value = (usize, usize, usize)> {
        (min..=max).prop_flat_map(|n| (0..n, 0..n)
            .prop_filter("v and w must be different", |&(v, w)| v != w)
            .prop_map(move |(v, w)| (n, v, w)))
    }

    proptest! {
        #[test]
        fn test_1_edge((n, v, w) in gen_graph_1_edge(3, 6)) {
            let mut g = Graph::new(n);
            g.add_edge(v, w);
            let ts = toposorts(g).collect::<HashSet<_>>();
            assert_eq!(ts.len(), factorial(n) - factorial(n - 2) * binomial(n, n - 2))
        }
    }
}
