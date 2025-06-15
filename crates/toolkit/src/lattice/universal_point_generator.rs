use crate::{
    aliases::{UniversalLatticePoint, UniversalLatticeSize},
    utils::point_from_flat_index,
};

pub struct UniversalPointGenerator<const D: usize> {
    remaining: usize,
    size: UniversalLatticeSize<D>,
    index: usize,
}

impl<const D: usize> UniversalPointGenerator<D> {
    pub fn new(until: UniversalLatticeSize<D>) -> Self {
        let index = until.values().iter().product();

        Self {
            remaining: index,
            size: until,
            index,
        }
    }
}

impl<const D: usize> Iterator for UniversalPointGenerator<D> {
    type Item = UniversalLatticePoint<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let index = self.index - self.remaining;
            self.remaining -= 1;
            Some(point_from_flat_index::<D>(index, self.size))
        }
    }
}
