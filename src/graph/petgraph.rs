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

impl<'a, G: NodeCount + NodeIndexable + IntoEdgeReferences + IntoNeighborsDirected> Graph for G {
    type Edges<'b> = Edges<'b, G> where G: 'b;
    type Successors<'b> = Successors<'b, G> where G: 'b;

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
    use crate::graph::Graph;
    use petgraph::prelude as petgraph;

    #[test]
    fn test_graph() {
        let mut g = petgraph::DiGraph::<(), ()>::new();
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
}
