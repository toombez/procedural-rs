use crate::{lattice::universal_lattice_point::UniversalLatticePoint, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniversalLatticeSize<const D: usize>(Point<D, usize>);

impl<const D: usize> UniversalLatticeSize<D> {
    pub fn new(coords: [usize; D]) -> Self {
        Self(Point::new(coords))
    }

    pub fn contains(&self, point: &UniversalLatticePoint<D>) -> bool {
        point.is_inside(&self)
    }
}

impl<const D: usize> Into<Point<D, usize>> for UniversalLatticeSize<D> {
    fn into(self) -> Point<D, usize> {
        self.0
    }
}

impl<const D: usize> IntoIterator for UniversalLatticeSize<D> {
    type Item = usize;
    type IntoIter = std::array::IntoIter<Self::Item, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
