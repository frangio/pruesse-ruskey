#[derive(Debug)]
pub struct Graph {
    succ: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        let succ = vec![vec![]; size];
        Graph { succ }
    }

    pub fn size(&self) -> usize {
        self.succ.len()
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.succ[from].push(to)
    }

    pub fn edges(&'_ self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.succ.iter().enumerate().flat_map(|(v, ws)| ws.iter().map(move |&w| (v, w)))
    }

    pub fn successors(&'_ self, v: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        self.succ[v].iter().copied()
    }
}
