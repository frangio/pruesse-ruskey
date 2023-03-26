#[derive(Debug, Clone)]
pub struct Graph {
    succ: Vec<Vec<usize>>,
    edge_count: usize,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        let succ = vec![vec![]; size];
        let edge_count = 0;
        Graph { succ, edge_count }
    }

    pub fn size(&self) -> usize {
        self.succ.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.succ[from].push(to);
        self.edge_count += 1;
    }

    pub fn edges(&'_ self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.succ.iter().enumerate().flat_map(|(v, ws)| ws.iter().map(move |&w| (v, w)))
    }

    pub fn successors(&'_ self, v: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        self.succ[v].iter().copied()
    }
}
