use crate::{
    lattice::{universal_lattice::UniversalLattice, universal_lattice_point::UniversalLatticePoint}, types::{Lattice, Neighborhood, NeighborhoodBuilder}
};

#[derive(Debug)]
pub struct MooreNeighborhood<S> {
    states: Vec<S>,
}

impl<S> Neighborhood for MooreNeighborhood<S> {
    type State = S;
    type Iter<'a>
        = std::slice::Iter<'a, S>
    where
        S: 'a;

    fn iter_states(&self) -> Self::Iter<'_> {
        self.states.iter()
    }
}

#[derive(Debug)]
pub struct MooreNeighborhoodBuilder;

impl<S: Clone + Default> NeighborhoodBuilder<UniversalLattice<2, S>> for MooreNeighborhoodBuilder {
    type Neighborhood = MooreNeighborhood<S>;

    fn build_neighborhood(
        &self,
        point: &<UniversalLattice<2, S> as crate::prelude::Lattice>::Point,
        lattice: &UniversalLattice<2, S>,
    ) -> Self::Neighborhood {
        let mut states = Vec::new();

        let (x, y) = (point.get(0).unwrap(), point.get(1).unwrap());

        let directions = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for (dx, dy) in directions.iter() {
            let neighbor_point = UniversalLatticePoint::new([x + dx, y + dy]);
            states.push(lattice.get_state(&neighbor_point));
        }

        MooreNeighborhood { states }
    }
}
