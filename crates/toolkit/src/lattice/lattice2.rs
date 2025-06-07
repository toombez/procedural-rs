use std::{collections::HashMap, fmt::Debug};

use nalgebra::{Const, OPoint};

use crate::{
    types::{BoundaryHandling, BoundaryHandlingLattice, Lattice},
    utils::{clamp_coordinate, wrap_coordinate},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lattice2Point(OPoint<i128, Const<2>>);

impl Lattice2Point {
    pub fn new(x: i128, y: i128) -> Self {
        Self(OPoint::<i128, Const<2>>::new(x, y))
    }

    pub fn x(&self) -> i128 {
        self.0.x
    }

    pub fn y(&self) -> i128 {
        self.0.y
    }
}

impl From<(i128, i128)> for Lattice2Point {
    fn from(value: (i128, i128)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Into<(i128, i128)> for Lattice2Point {
    fn into(self) -> (i128, i128) {
        (self.x(), self.y())
    }
}

impl From<(usize, usize)> for Lattice2Point {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as i128, value.1 as i128)
    }
}

impl Into<(usize, usize)> for Lattice2Point {
    fn into(self) -> (usize, usize) {
        (self.x() as usize, self.y() as usize)
    }
}

#[derive(Debug, Clone)]
pub struct Lattice2<D> {
    points: HashMap<Lattice2Point, D>,
    size: (usize, usize),
    boundary_handling: BoundaryHandling,
}

impl<D> Lattice2<D> {
    pub fn calculate_size(&mut self) {
        let sizes = self.points.keys().fold((0, 0), |(width, height), point| {
            let x = width.max(point.x() as usize);
            let y = height.max(point.y() as usize);

            (x, y)
        });

        self.size = sizes
    }
}

impl<D> From<HashMap<Lattice2Point, D>> for Lattice2<D> {
    fn from(points: HashMap<Lattice2Point, D>) -> Self {
        Self {
            points,
            boundary_handling: BoundaryHandling::Default,
            size: (0, 0),
        }
    }
}

impl<D: Clone + Default> BoundaryHandlingLattice for Lattice2<D> {
    type Size = (usize, usize);

    fn transform_point(&self, point: &Self::Point) -> Self::Point {
        let size = self.size();

        let (x, y) = (point.x(), point.y());

        if x >= 0 && (x as usize) < size.0 && y >= 0 && (y as usize) < size.1 {
            return *point;
        }

        match self.boundary_handling() {
            BoundaryHandling::Default => *point,
            BoundaryHandling::Clamp => {
                Lattice2Point::new(clamp_coordinate(x, size.0), clamp_coordinate(y, size.1))
            }
            BoundaryHandling::Wrap => {
                Lattice2Point::new(wrap_coordinate(x, size.0), wrap_coordinate(y, size.1))
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
}

impl<D: Clone + Default> Lattice for Lattice2<D> {
    type Point = Lattice2Point;
    type State = D;

    fn get_state(&self, point: &Self::Point) -> Self::State {
        let transformed = self.transform_point(point);

        self.points.get(&transformed).cloned().unwrap_or_default()
    }

    fn set_state(&mut self, point: &Self::Point, state: &Self::State) {
        self.points.insert(*point, state.clone());
    }

    fn points(&self) -> Vec<Self::Point> {
        let size = self.size();

        let mut points = vec![];

        for x in 0..size.0 {
            for y in 0..size.1 {
                points.push(Lattice2Point::from((x, y)));
            }
        }

        points
    }
}

impl<D> IntoIterator for Lattice2<D> {
    type Item = (Lattice2Point, D);
    type IntoIter = std::collections::hash_map::IntoIter<Lattice2Point, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}
