use std::collections::BTreeMap;

use crate::{
    lattice::lattice1_point::Lattice1Point,
    types::{BoundaryHandling, BoundaryHandlingLattice, Lattice},
    utils::{clamp_coordinate, wrap_coordinate},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Lattice1Size {
    width: usize,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Lattice1Size {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(width: usize) -> Self {
        Self { width }
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn width(&self) -> usize {
        self.width
    }
}

#[derive(Debug, Clone)]
pub struct Lattice1<D> {
    points: BTreeMap<Lattice1Point, D>,
    boundary_handling: BoundaryHandling,
    size: Lattice1Size,
}

impl <D> IntoIterator for Lattice1<D> {
    type Item = (Lattice1Point, D);
    type IntoIter = std::collections::btree_map::IntoIter<Lattice1Point, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

impl<D> From<Lattice1Size> for Lattice1<D>
where
    D: Clone + Default,
{
    fn from(size: Lattice1Size) -> Self {
        Self {
            boundary_handling: BoundaryHandling::default(),
            points: BTreeMap::new(),
            size,
        }
    }
}

impl<D> Lattice for Lattice1<D>
where
    D: Clone + Default,
{
    type Point = Lattice1Point;
    type State = D;

    fn get_state(&self, point: &Self::Point) -> Self::State {
        self.points.get(point).cloned().unwrap_or_default()
    }

    fn set_state(&mut self, point: &Self::Point, state: &Self::State) {
        self.points.insert(*point, state.clone());
    }

    fn points(&self) -> Vec<Self::Point> {
        self.points.keys().map(|point| point.clone()).collect()
    }

    fn states(&self) -> Vec<Self::State> {
        self.points()
            .iter()
            .map(|point| self.get_state(point))
            .collect()
    }
}

impl<D> BoundaryHandlingLattice for Lattice1<D>
where
    D: Clone + Default,
{
    type Size = Lattice1Size;

    fn from_states(states: Vec<Self::State>, size: Self::Size) -> Self {
        let mut lattice = Self::from(size);

        states.iter().enumerate().for_each(|(x, state)| {
            lattice.set_state(&Lattice1Point::new(x as i128), state);
        });

        lattice
    }

    fn transform_point(&self, point: &Self::Point) -> Self::Point {
        let Lattice1Size { width} = self.size();
        let x = point.x();

        let is_in_width = x >= 0 && (x as usize) < width;

        if is_in_width {
            return *point;
        }

        match self.boundary_handling() {
            BoundaryHandling::Default => *point,
            BoundaryHandling::Clamp => {
                let clamped_x = clamp_coordinate(x, width);
                Lattice1Point::new(clamped_x)
            }
            BoundaryHandling::Wrap => {
                let wrapped_x = wrap_coordinate(x, width);
                Lattice1Point::new(wrapped_x)
            }
        }
    }

    fn set_boundary_handling(&mut self, boundary_handling: BoundaryHandling) {
        self.boundary_handling = boundary_handling;
    }

    fn boundary_handling(&self) -> BoundaryHandling {
        self.boundary_handling
    }

    fn size(&self) -> Self::Size {
        self.size
    }

    fn set_size(&mut self, size: Self::Size) {
        self.size = size;
    }
}
