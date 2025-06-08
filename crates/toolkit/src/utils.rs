#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

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
