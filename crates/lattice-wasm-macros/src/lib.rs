#[macro_export]
macro_rules! lattice_wasm {
    ($type:ty, $point:ty, $name:ident, $lattice:ident) => {
        #[wasm_bindgen]
        pub struct $name {
            inner: $lattice,
        }

        #[wasm_bindgen]
        impl $name {
            // pub fn new(states: Vec<$type, size: $size) -> Self {}
            // pub fn from_size(size: $size) -> Self {}
            // pub fn from_states(states: Vec<$type>, size: $size) -> Self {}

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

            // pub fn size(&self) -> $size {}
            // pub fn set_size(&mut self, size: $size) {}
            // pub fn transform_point(&self, point: &$point) -> $point {}
        }
    };
}
