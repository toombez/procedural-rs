pub trait Lattice<P, S> {
    fn get_state(&self, point: P) -> S;
}

pub trait Neighborhood<P, S> {
    fn get_neighbors(point: P, lattice: impl Lattice<P, S>) -> Self;
}

pub trait CellularAutomata<P, S> {
    fn evolve(&mut self);
    fn evolve_point(&self, point: P) -> S;
}
