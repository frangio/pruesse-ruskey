#![cfg(feature = "petgraph")]

use super::Graph;
use petgraph::Direction;
use petgraph::visit::{NodeCount, NodeIndexable, IntoEdgeReferences, IntoNeighborsDirected, EdgeRef};

pub struct Edges<'a, G: NodeIndexable + IntoEdgeReferences> {
    graph: &'a G,
    edges: <&'a G as IntoEdgeReferences>::EdgeReferences,
}

impl<'a, G: NodeIndexable + IntoEdgeReferences> Iterator for Edges<'a, G> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next().map(|e| {
            let v = self.graph.to_index(e.source());
            let w = self.graph.to_index(e.target());
            (v, w)
        })
    }
}

pub struct Successors<'a, G: NodeIndexable + IntoNeighborsDirected> {
    graph: &'a G,
    neighbors: <&'a G as IntoNeighborsDirected>::NeighborsDirected,
}

impl<'a, G: NodeIndexable + IntoNeighborsDirected> Iterator for Successors<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.neighbors.next().map(|v| self.graph.to_index(v))
    }
}

impl<G: NodeCount + NodeIndexable + IntoEdgeReferences + IntoNeighborsDirected> Graph for G {
    type Edges<'a> = Edges<'a, G> where G: 'a;
    type Successors<'a> = Successors<'a, G> where G: 'a;

    fn size(&self) -> usize {
        self.node_count()
    }

    fn edges(&self) -> Self::Edges<'_> {
        Edges { graph: self, edges: self.edge_references() }
    }

    fn successors(&self, v: usize) -> Self::Successors<'_> {
        let neighbors = self.neighbors_directed(self.from_index(v), Direction::Outgoing);
        Successors { graph: self, neighbors }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::prelude::*;
    use crate::graph::Graph;

    #[test]
    fn test_size() {
        let n = 4;

        let mut g = DiGraph::<(), ()>::new();
        for _ in 0..n {
            g.add_node(());
        }

        let size = Graph::size(&&g);
        assert_eq!(size, n);
    }

    #[test]
    fn test_edges() {
        let mut g = DiGraph::<(), ()>::new();
        for _ in 0..4 {
            g.add_node(());
        }

        let edges: Vec<_> = Graph::edges(&&g).collect();
        assert_eq!(edges, vec![]);

        g.add_edge(0.into(), 1.into(), ());
        g.add_edge(1.into(), 2.into(), ());
        g.add_edge(2.into(), 3.into(), ());

        let edges: Vec<_> = Graph::edges(&&g).collect();
        assert_eq!(edges, vec![(0, 1), (1, 2), (2, 3)]);
    }

    #[test]
    fn test_successors() {
        let mut g = DiGraph::<(), ()>::new();
        for _ in 0..3 {
            g.add_node(());
        }

        g.add_edge(0.into(), 1.into(), ());
        g.add_edge(0.into(), 2.into(), ());

        let mut successors: Vec<_> = Graph::successors(&&g, 0).collect();
        successors.sort();
        assert_eq!(successors, vec![1, 2]);
    }
}
