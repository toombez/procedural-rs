#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    aliases::{UniversalLatticePoint, UniversalLatticeSize},
    lattice::universal_point::UniversalPoint,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn wrap_coordinate(coord: i128, size: usize) -> i128 {
    if size == 0 {
        return 0;
    }

    let size_i128 = size as i128;
    (coord % size_i128 + size_i128) % size_i128
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn clamp_coordinate(coord: i128, size: usize) -> i128 {
    if size == 0 {
        return 0;
    }

    let max = size as i128 - 1;
    coord.clamp(0, max)
}

pub fn point_from_flat_index<const D: usize>(
    index: usize,
    size: UniversalLatticeSize<D>,
) -> UniversalLatticePoint<D> {
    let mut remaining = index;
    let mut coords = [0; D];

    for (index, dim) in size.into_iter().enumerate() {
        coords[index] = remaining % dim;
        remaining /= dim;

        if remaining == 0 {
            break;
        }
    }

    UniversalPoint::from_iter(coords.map(|coord| coord as i128))
}

pub fn generate_points_until<const D: usize>(
    size: UniversalLatticeSize<D>,
) -> Vec<UniversalLatticePoint<D>> {
    (0..size.values().iter().product())
        .map(|index| point_from_flat_index(index, size))
        .collect()
}
