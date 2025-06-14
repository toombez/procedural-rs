use std::collections::BTreeMap;

use crate::{
    lattice::{
        universal_lattice_point::UniversalLatticePoint,
        universal_lattice_size::UniversalLatticeSize,
    },
    types::{BoundaryHandling, BoundaryHandlingLattice, Lattice},
    utils::{clamp_coordinate, wrap_coordinate},
};

#[derive(Debug, Clone)]
pub struct UniversalLattice<const D: usize, S> {
    points: BTreeMap<UniversalLatticePoint<D>, S>,
    size: UniversalLatticeSize<D>,
    boundary_handling: BoundaryHandling,
}

impl<const D: usize, S> From<UniversalLatticeSize<D>> for UniversalLattice<D, S> {
    fn from(size: UniversalLatticeSize<D>) -> Self {
        Self {
            points: BTreeMap::new(),
            size,
            boundary_handling: BoundaryHandling::default(),
        }
    }
}

impl<const D: usize, S> IntoIterator for UniversalLattice<D, S> {
    type Item = (UniversalLatticePoint<D>, S);
    type IntoIter = std::collections::btree_map::IntoIter<UniversalLatticePoint<D>, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

impl<const D: usize, S: Clone + Default> Lattice for UniversalLattice<D, S> {
    type Point = UniversalLatticePoint<D>;
    type State = S;

    fn get_state(&self, point: &Self::Point) -> Self::State {
        let transformed = self.transform_point(point);
        self.points.get(&transformed).cloned().unwrap_or_default()
    }

    fn set_state(&mut self, point: &Self::Point, state: &Self::State) {
        let transformed = self.transform_point(point);

        if !transformed.is_inside(&self.size()) {
            return
        }

        self.points.insert(transformed, state.clone());
    }

    fn points(&self) -> Vec<Self::Point> {
        self.points.keys().map(|point| *point).collect()
    }

    fn states(&self) -> Vec<Self::State> {
        self.points.values().map(|state| state.clone()).collect()
    }
}

impl<const D: usize, S: Clone + Default> BoundaryHandlingLattice for UniversalLattice<D, S> {
    type Size = UniversalLatticeSize<D>;

    fn from_states(states: Vec<Self::State>, size: Self::Size) -> Self {
        let mut lattice = Self::from(size);

        states.into_iter().enumerate().for_each(|(index, state)| {
            let point = UniversalLatticePoint::from((index, size));
            lattice.set_state(&point, &state);
        });

        lattice
    }

    fn transform_point(&self, point: &Self::Point) -> Self::Point {
        if point.is_inside(&self.size) {
            return *point;
        }

        let coords_cb = match self.boundary_handling() {
            BoundaryHandling::Default => return *point,
            BoundaryHandling::Clamp => |(coord, size)| clamp_coordinate(coord, size as usize),
            BoundaryHandling::Wrap => |(coord, size)| wrap_coordinate(coord, size as usize),
        };

        let mut coords = point.coords();
        point
            .into_iter()
            .zip(self.size)
            .map(coords_cb)
            .enumerate()
            .for_each(|(index, coord)| coords[index] = coord);

        UniversalLatticePoint::new(coords)
    }

    fn set_boundary_handling(&mut self, boundary_handling: BoundaryHandling) {
        self.boundary_handling = boundary_handling
    }

    fn boundary_handling(&self) -> BoundaryHandling {
        self.boundary_handling
    }

    fn size(&self) -> Self::Size {
        self.size
    }

    fn set_size(&mut self, size: Self::Size) {
        self.size = size;
        self
            .points()
            .iter()
            .filter(|point| !point.is_inside(&size))
            .for_each(|point| {
                self.points.remove(point);
            });
    }
}
