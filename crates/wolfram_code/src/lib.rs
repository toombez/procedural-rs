pub mod state;
pub mod rule;
pub mod automaton;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct ATuple(pub i32, pub bool);

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct A {
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl A {
    #[wasm_bindgen]
    pub fn a() -> Vec<ATuple> {
        vec![]
    }
}
