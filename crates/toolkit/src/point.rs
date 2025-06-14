use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<const D: usize, C>([C; D]);

impl<const D: usize, C> Point<D, C> {
    pub fn new(coords: [C; D]) -> Self {
        Self(coords)
    }

    pub fn get(&self, index: usize) -> Option<&C> {
        self.0.get(index)
    }
}

impl<const D: usize, C: Clone> Point<D, C> {
    pub fn coords(&self) -> [C; D] {
        self.0.clone()
    }
}

impl<const D: usize, C> Index<usize> for Point<D, C> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0.index(index)
    }
}

impl<const D: usize, C> IndexMut<usize> for Point<D, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const D: usize, C> IntoIterator for Point<D, C> {
    type Item = C;
    type IntoIter = std::array::IntoIter<C, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
