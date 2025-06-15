use crate::lattice::universal_point::UniversalPoint;

pub type UniversalLatticePoint<const D: usize> = UniversalPoint<i128, D>;
pub type UniversalLatticeSize<const D: usize> = UniversalPoint<usize, D>;
