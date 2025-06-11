use nalgebra::{Const, OPoint};
use std::cmp::Ordering;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Lattice2Point(OPoint<i128, Const<2>>);

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Lattice2Point {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(x: i128, y: i128) -> Self {
        Self(OPoint::<i128, Const<2>>::new(x, y))
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn x(&self) -> i128 {
        self.0.x
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
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

impl Ord for Lattice2Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y().cmp(&other.y()) {
            Ordering::Less | Ordering::Equal => self.x().cmp(&other.x()),
            Ordering::Greater => Ordering::Greater,
        }
    }
}
