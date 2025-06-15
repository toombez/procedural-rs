use crate::{
    aliases::UniversalLatticePoint, lattice::universal_lattice::UniversalLattice, prelude::*,
};

#[derive(Debug, Clone)]
pub struct NearestNeighborhood1<S> {
    states: Vec<S>,
}

impl<S> Neighborhood for NearestNeighborhood1<S> {
    type State = S;
    type Iter<'a>
        = std::slice::Iter<'a, S>
    where
        Self: 'a;

    fn iter_states(&self) -> Self::Iter<'_> {
        self.states.iter()
    }
}

#[derive(Debug, Clone)]
pub struct NearestNeighborhoodBuilder1 {
    radius: usize,
}

impl NearestNeighborhoodBuilder1 {
    pub fn new(radius: usize) -> Self {
        Self { radius }
    }
}

impl From<usize> for NearestNeighborhoodBuilder1 {
    fn from(radius: usize) -> Self {
        Self::new(radius)
    }
}

impl<S: Clone + Default> NeighborhoodBuilder<UniversalLattice<1, S>>
    for NearestNeighborhoodBuilder1
{
    type Neighborhood = NearestNeighborhood1<S>;

    fn build_neighborhood(
        &self,
        point: &<UniversalLattice<1, S> as Lattice>::Point,
        lattice: &UniversalLattice<1, S>,
    ) -> Self::Neighborhood {
        let mut states = Vec::new();

        for offset in -(self.radius as i128)..=(self.radius as i128) {
            if offset == 0 {
                continue;
            }

            let x = point.get(0).unwrap();

            let neighbor_point = UniversalLatticePoint::new([x + offset]);
            let state = lattice.get_state(&neighbor_point);

            states.push(state);
        }

        NearestNeighborhood1 { states }
    }
}
