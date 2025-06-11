use crate::{
    lattice::lattice2_point::Lattice2Point,
    types::{BoundaryHandling, BoundaryHandlingLattice, Lattice},
    utils::{clamp_coordinate, wrap_coordinate},
};
use std::collections::BTreeMap;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Lattice2Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct Lattice2<D> {
    points: BTreeMap<Lattice2Point, D>,
    size: Lattice2Size,
    boundary_handling: BoundaryHandling,
}

impl<D> From<Lattice2Size> for Lattice2<D>
where
    D: Clone + Default,
{
    fn from(size: Lattice2Size) -> Self {
        Self {
            boundary_handling: BoundaryHandling::default(),
            points: BTreeMap::new(),
            size,
        }
    }
}

impl<D> Lattice for Lattice2<D>
where
    D: Clone + Default,
{
    type Point = Lattice2Point;
    type State = D;

    fn get_state(&self, point: &Self::Point) -> Self::State {
        self.points.get(point).cloned().unwrap_or_default()
    }

    fn set_state(&mut self, point: &Self::Point, state: &Self::State) {
        self.points.insert(*point, state.clone());
    }

    fn points(&self) -> Vec<Self::Point> {
        self.points.keys().map(|point| *point).collect()
    }

    fn states(&self) -> Vec<Self::State> {
        self.points.iter().map(|(_, state)| state.clone()).collect()
    }
}

impl<D> BoundaryHandlingLattice for Lattice2<D>
where
    D: Clone + Default,
{
    type Size = Lattice2Size;

    fn from_states(states: Vec<Self::State>, size: Self::Size) -> Self {
        let mut lattice = Self::from(size);

        states.iter().enumerate().for_each(|(idx, state)| {
            let x = (idx / size.height) as i128;
            let y = (idx % size.width) as i128;

            let point = Lattice2Point::new(x, y);

            lattice.set_state(&point, state);
        });

        lattice
    }

    fn transform_point(&self, point: &Self::Point) -> Self::Point {
        let Lattice2Size { width, height } = self.size();
        let (x, y) = (point.x(), point.y());

        let is_in_width = x >= 0 && (x as usize) < width;
        let is_in_height = y >= 0 && (y as usize) < height;

        if is_in_width && is_in_height {
            return *point;
        }

        match self.boundary_handling() {
            BoundaryHandling::Default => *point,
            BoundaryHandling::Clamp => {
                let clamped_x = clamp_coordinate(x, width);
                let clamped_y = clamp_coordinate(y, height);
                Lattice2Point::new(clamped_x, clamped_y)
            }
            BoundaryHandling::Wrap => {
                let wrapped_x = wrap_coordinate(x, width);
                let wrapped_y = wrap_coordinate(y, height);
                Lattice2Point::new(wrapped_x, wrapped_y)
            }
        }
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
        self.size = size
    }
}
