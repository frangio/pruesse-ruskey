use std::{slice, iter};

use super::Graph;

#[derive(Debug, Clone)]
pub struct SimpleGraph {
    succ: Vec<Vec<usize>>,
    edge_count: usize,
}

impl SimpleGraph {
    pub fn new(size: usize) -> Self {
        let succ = vec![vec![]; size];
        let edge_count = 0;
        SimpleGraph { succ, edge_count }
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(to < self.size());
        self.succ[from].push(to);
        self.edge_count += 1;
    }
}

pub struct Edges<'a>(&'a SimpleGraph, usize, usize);

impl<'a> Iterator for Edges<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let Edges(g, v, i) = self;
        loop {
            let ws = g.succ.get(*v)?;
            if let Some(w) = ws.get(*i) {
                *i += 1;
                return Some((*v, *w))
            } else {
                *v += 1;
                *i = 0;
            }
        }
    }
}

impl Graph for SimpleGraph {
    type Edges<'a> = Edges<'a>;
    type Successors<'a> = iter::Copied<slice::Iter<'a, usize>>;

    fn size(&self) -> usize {
        self.succ.len()
    }

    fn edges(&self) -> Self::Edges<'_> {
        Edges(self, 0, 0)
    }

    fn successors(&self, v: usize) -> Self::Successors<'_> {
        self.succ[v].iter().copied()
    }
}
