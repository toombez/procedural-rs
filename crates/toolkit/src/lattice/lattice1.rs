use std::fmt::Debug;

use nalgebra::{Const, OPoint};

use crate::{
    types::{BoundaryHandling, BoundaryHandlingLattice, Lattice},
    utils::{clamp_coordinate, wrap_coordinate},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature="wasm", wasm_bindgen)]
pub struct Lattice1Point(OPoint<i128, Const<1>>);

#[cfg_attr(feature="wasm", wasm_bindgen)]
impl Lattice1Point {
    #[cfg_attr(feature="wasm", wasm_bindgen(constructor))]
    pub fn new(x: i128) -> Self {
        Self(OPoint::<i128, Const<1>>::new(x))
    }

    #[cfg_attr(feature="wasm", wasm_bindgen(getter))]
    pub fn x(&self) -> i128 {
        self.0.x
    }
}

impl From<usize> for Lattice1Point {
    fn from(x: usize) -> Self {
        Self::new(x as i128)
    }
}

impl Into<usize> for Lattice1Point {
    fn into(self) -> usize {
        self.x() as usize
    }
}

impl From<i128> for Lattice1Point {
    fn from(x: i128) -> Self {
        Self::new(x)
    }
}

impl Into<i128> for Lattice1Point {
    fn into(self) -> i128 {
        self.x()
    }
}

#[derive(Debug, Clone)]
pub struct Lattice1<D> {
    points: Vec<D>,
    boundary_handling: BoundaryHandling,
}

impl<D> From<Vec<D>> for Lattice1<D> {
    fn from(points: Vec<D>) -> Self {
        Self {
            points,
            boundary_handling: BoundaryHandling::Default,
        }
    }
}

impl<D: Default + Clone> From<usize> for Lattice1<D> {
    fn from(size: usize) -> Self {
        Self::from(vec![D::default(); size])
    }
}

impl<D: Default + Clone> BoundaryHandlingLattice for Lattice1<D> {
    type Size = usize;

    fn transform_point(&self, point: &Self::Point) -> Self::Point {
        let x = point.x();
        let size = self.size();

        match self.boundary_handling() {
            BoundaryHandling::Default => *point,
            BoundaryHandling::Clamp => Lattice1Point::from(clamp_coordinate(x, size)),
            BoundaryHandling::Wrap => Lattice1Point::from(wrap_coordinate(x, size)),
        }
    }

    fn set_boundary_handling(&mut self, boundary_handling: BoundaryHandling) {
        self.boundary_handling = boundary_handling;
    }

    fn boundary_handling(&self) -> BoundaryHandling {
        self.boundary_handling
    }

    fn size(&self) -> Self::Size {
        self.points.len()
    }
}

impl<D: Default + Clone> Lattice for Lattice1<D> {
    type Point = Lattice1Point;
    type State = D;

    fn get_state(&self, point: &Self::Point) -> Self::State {
        let transformed = self.transform_point(point);
        let idx = transformed.x() as usize;

        self.points.get(idx).cloned().unwrap_or_default()
    }

    fn set_state(&mut self, point: &Self::Point, state: &Self::State) {
        let transformed = self.transform_point(point);
        let idx = transformed.x() as usize;

        if idx < self.size() {
            self.points[idx] = state.clone()
        }
    }

    fn points(&self) -> Vec<Self::Point> {
        (0..self.size())
            .map(|point| Lattice1Point::new(point as i128))
            .collect()
    }
}

impl<D: Default + Clone> IntoIterator for Lattice1<D> {
    type Item = (Lattice1Point, D);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let points = self
            .points
            .iter()
            .enumerate()
            .map(|(point, state)| (Lattice1Point::new(point as i128), state.clone()))
            .collect::<Vec<Self::Item>>();

        points.into_iter()
    }
}
