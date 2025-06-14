use crate::{lattice::universal_lattice_size::UniversalLatticeSize, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniversalLatticePoint<const D: usize>(Point<D, i128>);

impl<const D: usize> UniversalLatticePoint<D> {
    pub fn new(coords: [i128; D]) -> Self {
        Self(Point::new(coords))
    }

    pub fn get(&self, index: usize) -> Option<&i128> {
        self.0.get(index)
    }

    pub fn coords(&self) -> [i128; D] {
        self.0.coords()
    }
}

impl<const D: usize> From<(usize, UniversalLatticeSize<D>)> for UniversalLatticePoint<D> {
    fn from((index, size): (usize, UniversalLatticeSize<D>)) -> Self {
        let size: Point<D, usize> = size.into();
        let mut remaining = index;
        let mut coords = [0; D];

        for (index, dim) in size.into_iter().enumerate() {
            coords[index] = (remaining % dim) as i128;
            remaining /= dim;

            if remaining == 0 {
                break;
            }
        }

        Self::new(coords)
    }
}

impl<const D: usize> IntoIterator for UniversalLatticePoint<D> {
    type Item = i128;
    type IntoIter = std::array::IntoIter::<Self::Item, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const D: usize> Into<Point<D, i128>> for UniversalLatticePoint<D> {
    fn into(self) -> Point<D, i128> {
        self.0
    }
}
