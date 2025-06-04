pub trait Lattice {
    type Point;
    type State;

    fn get_state(&self, point: &Self::Point) -> Self::State;
    fn set_state(&mut self, point: &Self::Point, state: &Self::State);
}

pub enum Overload<S> {
    Outside(S),
    Inside,
}

pub trait BoundedLattice<S, L: Lattice> {
    fn get_bounds(&self) -> L::Point;
    fn is_overload(&self, point: &L::Point) -> Overload<S>;
    fn adjust_point(&self, point: &L::Point) -> L::Point;
}

pub trait Neighborhood<L: Lattice> {
    fn get_neighbors(point: &L::Point, lattice: &L) -> Self;
}

pub trait Rule<L: Lattice, N: Neighborhood<L>> {
    fn transform_state(&self, state: &L::State, neighborhood: &N) -> L::State;
}

pub trait CellularAutomataMarker {}

pub trait CellularAutomata<L: Lattice, N: Neighborhood<L>, R: Rule<L, N>> {
    fn evolve(&self, lattice: &L, rule: &R) -> L;
}

impl<S, L, N, R> CellularAutomata<L, N, R> for S
where
    S: CellularAutomataMarker,
    L: Lattice + IntoIterator<Item = (L::Point, L::State)> + Clone,
    N: Neighborhood<L>,
    R: Rule<L, N>,
{
    fn evolve(&self, lattice: &L, rule: &R) -> L {
        let mut lattice_copy = lattice.clone();

        lattice.clone().into_iter().for_each(|(point, state)| {
            let neighborhood = N::get_neighbors(&point, &lattice);
            let new_state = rule.transform_state(&state, &neighborhood);

            lattice_copy.set_state(&point, &new_state);
        });

        lattice_copy
    }
}
