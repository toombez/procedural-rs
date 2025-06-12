use std::cmp::Ordering;

use nalgebra::{Const, OPoint};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Lattice3Point(OPoint<i128, Const<3>>);

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Lattice3Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i128, y: i128, z: i128) -> Self {
        Self(OPoint::<i128, Const<3>>::new(x, y, z))
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i128 {
        self.0.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i128 {
        self.0.y
    }

    #[wasm_bindgen(getter)]
    pub fn z(&self) -> i128 {
        self.0.z
    }
}

impl Ord for Lattice3Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.z().cmp(&other.z()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.y().cmp(&other.y()) {
                Ordering::Equal => self.x().cmp(&other.x()),
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            },
        }
    }
}
