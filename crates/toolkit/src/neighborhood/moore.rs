use crate::{
    lattice::{lattice2::Lattice2, lattice2_point::Lattice2Point},
    types::{Lattice, Neighborhood, NeighborhoodBuilder},
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

impl<S: Clone + Default> NeighborhoodBuilder<Lattice2<S>> for MooreNeighborhoodBuilder {
    type Neighborhood = MooreNeighborhood<S>;

    fn build_neighborhood(
        &self,
        point: &<Lattice2<S> as crate::prelude::Lattice>::Point,
        lattice: &Lattice2<S>,
    ) -> Self::Neighborhood {
        let mut states = Vec::new();

        let (x, y) = (point.x(), point.y());

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
            let neighbor_point = Lattice2Point::new(x + dx, y + dy);
            states.push(lattice.get_state(&neighbor_point));
        }

        MooreNeighborhood { states }
    }
}
