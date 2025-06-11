use nalgebra::{Const, OPoint};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Lattice1Point(OPoint<i128, Const<1>>);

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Lattice1Point {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(x: i128) -> Self {
        Self(OPoint::<i128, Const<1>>::new(x))
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
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

impl Ord for Lattice1Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x().cmp(&other.x())
    }
}
