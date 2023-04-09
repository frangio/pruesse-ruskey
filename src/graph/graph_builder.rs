#![cfg(feature = "graph_builder")]

use graph_builder as gb;
use std::iter;
use super::Graph;

pub struct Edges<'a, G: gb::Graph<usize> + gb::DirectedNeighbors<usize>> {
    graph: &'a G,
    curr: usize,
    edges: <G as gb::DirectedNeighbors<usize>>::NeighborsIterator<'a>,
}

impl<'a, G: gb::Graph<usize> + gb::DirectedNeighbors<usize>> Edges<'a, G> {
    fn new(graph: &'a G) -> Self {
        let curr = 0;
        Edges { graph, curr, edges: graph.out_neighbors(curr) }
    }
}

impl<'a, G: gb::Graph<usize> + gb::DirectedNeighbors<usize>> Iterator for Edges<'a, G> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(&w) = self.edges.next() {
                return Some((self.curr, w))
            } else if self.curr + 1 >= self.graph.size() {
                return None
            } else {
                self.curr += 1;
                self.edges = self.graph.out_neighbors(self.curr);
            }
        }
    }
}

impl<G: gb::Graph<usize> + gb::DirectedNeighbors<usize>> Graph for G {
    type Edges<'a> = Edges<'a, G> where Self: 'a;
    type Successors<'a> = iter::Copied<<Self as gb::DirectedNeighbors<usize>>::NeighborsIterator<'a>> where G: 'a;

    fn size(&self) -> usize {
        self.node_count()
    }

    fn edges(&self) -> Self::Edges<'_> {
        Edges::new(self)
    }

    fn successors(&self, v: usize) -> Self::Successors<'_> {
        self.out_neighbors(v).copied()
    }
}

#[cfg(test)]
mod test {
    use graph_builder::prelude::*;
    use crate::graph::Graph;

    #[test]
    fn test_size() {
        let g: DirectedCsrGraph<usize> = GraphBuilder::new()
            .edges(vec![(0, 1), (1, 2), (2, 3)])
            .build();

        let size = Graph::size(&g);
        assert_eq!(size, 4);
    }

    #[test]
    fn test_edges() {
        let g: DirectedCsrGraph<usize> = GraphBuilder::new()
            .edges(vec![(0, 1), (1, 2)])
            .build();

        let edges: Vec<_> = Graph::edges(&g).collect();
        assert_eq!(edges, vec![(0, 1), (1, 2)]);
    }
}
