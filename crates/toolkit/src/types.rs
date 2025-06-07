use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoundaryHandling {
    #[default]
    Default,
    Wrap,
    Clamp,
}

pub trait Lattice: IntoIterator {
    type Point;
    type State;

    fn get_state(&self, point: &Self::Point) -> Self::State;
    fn set_state(&mut self, point: &Self::Point, state: &Self::State);
    fn points(&self) -> Vec<Self::Point>;
}

pub trait BoundaryHandlingLattice: Lattice
where
    Self::Point: Clone,
{
    type Size: Copy;

    fn transform_point(&self, point: &Self::Point) -> Self::Point;
    fn set_boundary_handling(&mut self, boundary_handling: BoundaryHandling);
    fn boundary_handling(&self) -> BoundaryHandling;
    fn size(&self) -> Self::Size;
}

pub trait Neighborhood {
    type State;
    type Iter<'a>: Iterator<Item = &'a Self::State>
    where
        Self: 'a;

    fn iter_states(&self) -> Self::Iter<'_>;
}

pub trait NeighborhoodBuilder<L: Lattice> {
    type Neighborhood: Neighborhood<State = L::State>;

    fn build_neighborhood(point: &L::Point, lattice: &L) -> Self::Neighborhood;
}

pub trait Rule {
    type State;

    fn apply(
        &self,
        current_state: &Self::State,
        neighbors: &impl Neighborhood<State = Self::State>,
    ) -> Self::State;
}

pub trait CellularAutomaton {
    type Lattice: Lattice;
    type Rule: Rule<State = <Self::Lattice as Lattice>::State>;
    type NeighborhoodBuilder: NeighborhoodBuilder<
        Self::Lattice,
        Neighborhood: Neighborhood<State = <Self::Lattice as Lattice>::State>,
    >;

    fn rule(&self) -> &Self::Rule;

    fn step(&self, lattice: &mut Self::Lattice) {
        let points = lattice.points();

        let mut new_states = Vec::with_capacity(points.len());
        let rule = self.rule();

        for point in &points {
            let neighborhood = Self::NeighborhoodBuilder::build_neighborhood(&point, lattice);
            let current_state = lattice.get_state(&point);

            let new_state = rule.apply(&current_state, &neighborhood);
            new_states.push(new_state);
        }

        // Применяем вычисленные состояния к решетке
        for (point, new_state) in points.into_iter().zip(new_states) {
            lattice.set_state(&point, &new_state);
        }
    }
}
