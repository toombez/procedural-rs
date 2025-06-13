use std::collections::BTreeMap;
use crate::lattice::lattice3_point::Lattice3Point;
use crate::types::BoundaryHandlingLattice;
use crate::prelude::*;
use crate::utils::{clamp_coordinate, wrap_coordinate};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Lattice3Size {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Lattice3Size {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self { width, height, depth }
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn width(&self) -> usize {
        self.width
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn height(&self) -> usize {
        self.height
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn depth(&self) -> usize {
        self.depth
    }
}

#[derive(Debug, Clone)]
pub struct Lattice3<D> {
    points: BTreeMap<Lattice3Point, D>,
    boundary_handling: BoundaryHandling,
    size: Lattice3Size,
}

impl <D> From<Lattice3Size> for Lattice3<D>  {
    fn from(size: Lattice3Size) -> Self {
        Self {
            boundary_handling: BoundaryHandling::default(),
            points: BTreeMap::new(),
            size,
        }
    }
}

impl <D> IntoIterator for Lattice3<D> {
    type Item = (Lattice3Point, D);
    type IntoIter = std::collections::btree_map::IntoIter<Lattice3Point, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

impl <D: Clone + Default> Lattice for Lattice3<D> {
    type Point = Lattice3Point;
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

impl <D: Clone + Default> BoundaryHandlingLattice for Lattice3<D> {
    type Size = Lattice3Size;

    fn from_states(states: Vec<Self::State>, size: Self::Size) -> Self {
        let mut lattice = Self::from(size);

        states.iter().enumerate().for_each(|(idx, state)| {
            let z = (idx / (size.width() * size.height())) as i128;
            let y = (idx % (size.width() * size.height()) % size.width()) as i128;
            let x = (idx % (size.width() * size.height()) / size.width()) as i128;

            let point = Lattice3Point::new(x, y, z);

            lattice.set_state(&point, state);
        });

        lattice
    }

    fn transform_point(&self, point: &Self::Point) -> Self::Point {
        let Lattice3Size {
            depth,
            height,
            width,
        } = self.size;

        let (x, y, z) = (point.x(), point.y(), point.z());

        let is_in_width = x >= 0 && (x as usize) < width;
        let is_in_height = y >= 0 && (y as usize) < height;
        let is_in_depth = z >= 0 && (z as usize) < depth;

        if is_in_width && is_in_height && is_in_depth {
            return *point
        }

        match self.boundary_handling {
            BoundaryHandling::Default => *point,
            BoundaryHandling::Wrap => {
                let clamped_x = clamp_coordinate(x, width);
                let clamped_y = clamp_coordinate(y, height);
                let clamped_z = clamp_coordinate(z, depth);

                Lattice3Point::new(clamped_x, clamped_y, clamped_z)
            },
            BoundaryHandling::Clamp => {
                let wrapped_x = wrap_coordinate(x, width);
                let wrapped_y = wrap_coordinate(x, height);
                let wrapped_z = wrap_coordinate(x, depth);

                Lattice3Point::new(wrapped_x, wrapped_y, wrapped_z)
            },
        }
    }

    fn set_boundary_handling(&mut self, boundary_handling: crate::prelude::BoundaryHandling) {
        self.boundary_handling = boundary_handling;
    }

    fn boundary_handling(&self) -> crate::prelude::BoundaryHandling {
        self.boundary_handling
    }

    fn size(&self) -> Self::Size {
        self.size
    }

    fn set_size(&mut self, size: Self::Size) {
        self.size = size;
    }
}

