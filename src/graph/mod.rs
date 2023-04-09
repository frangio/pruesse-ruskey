pub mod simple;

pub trait Graph {
    type Edges<'a>: Iterator<Item = (usize, usize)> where Self: 'a;
    type Successors<'a>: Iterator<Item = usize> where Self: 'a;
    fn size(&self) -> usize;
    fn edges(&self) -> Self::Edges<'_>;
    fn successors(&self, v: usize) -> Self::Successors<'_>;
}
