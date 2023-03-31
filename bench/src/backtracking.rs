use topogen::graph::Graph;

#[derive(Debug)]
pub struct Traversals {
    graph: Graph,
    visits: Vec<Visit>,
    next: Vec<usize>,
    deps: Vec<isize>,
}

#[derive(Debug)]
struct Visit {
    node: usize,
    choice: usize,
}

impl Traversals {
    pub fn new(graph: Graph) -> Self {
        let n = graph.size();

        let visits = Vec::with_capacity(graph.size());
        let mut deps = vec![0; n];

        for (_, w) in graph.edges() {
            deps[w] += 1;
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

        for w in self.graph.successors(v) {
            self.deps[w] -= 1;
            if self.deps[w] == 0 {
                self.next.push(w);
            }
        }
    }

    fn backtrack(&mut self) -> usize {
        let Visit { node: v, choice } = self.visits.pop().unwrap();

        for w in self.graph.successors(v).rev() {
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

impl<'a> Iterator for Traversals {
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
