#[macro_export]
macro_rules! lattice_wasm {
    ($type:ty, $point:ty, $name:ident, $lattice:ident, $size:ident) => {
        #[cfg_attr(feature = "wasm", wasm_bindgen)]
        #[derive(Debug, Clone)]
        pub struct $name {
            inner: $lattice,
        }

        #[cfg(feature = "wasm")]
        #[wasm_bindgen]
        impl $name {
            #[wasm_bindgen(constructor)]
            pub fn new(states: Vec<$type>, size: $size) -> Self {
                Self::from_states(states, size)
            }

            #[wasm_bindgen]
            pub fn from_states(states: Vec<$type>, size: $size) -> Self {
                Self {
                    inner: $lattice::from_states(states, size),
                }
            }

            #[wasm_bindgen]
            pub fn from_size(size: $size) -> Self {
                Self {
                    inner: $lattice::from(size),
                }
            }

            #[wasm_bindgen]
            pub fn get_state(&self, point: &$point) -> $type {
                self.inner.get_state(point)
            }

            #[wasm_bindgen]
            pub fn set_state(&mut self, point: &$point, state: $type) {
                self.inner.set_state(point, &state);
            }

            #[wasm_bindgen(getter)]
            pub fn points(&self) -> Vec<$point> {
                self.inner.points()
            }

            #[wasm_bindgen(getter)]
            pub fn states(&self) -> Vec<$type> {
                self.inner.states()
            }

            #[wasm_bindgen]
            pub fn set_boundary_handling(&mut self, boundary_handling: BoundaryHandling) {
                self.inner.set_boundary_handling(boundary_handling);
            }

            #[wasm_bindgen(getter)]
            pub fn boundary_handling(&self) -> BoundaryHandling {
                self.inner.boundary_handling()
            }

            #[wasm_bindgen]
            pub fn size(&self) -> $size {
                self.inner.size()
            }

            #[wasm_bindgen]
            pub fn set_size(&mut self, size: $size) {
                self.inner.set_size(size);
            }

            #[wasm_bindgen]
            pub fn transform_point(&self, point: &$point) -> $point {
                self.inner.transform_point(point)
            }
        }

        impl From<$lattice> for $name {
            fn from(value: $lattice) -> Self {
                Self {
                    inner: value
                }
            }
        }

        impl Into<$lattice> for $name {
            fn into(self) -> $lattice {
                self.inner
            }
        }
    };
}
