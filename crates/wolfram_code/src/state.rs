#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum WolframCodeState {
    #[default]
    Dead,
    Alive,
}

impl Into<bool> for WolframCodeState {
    fn into(self) -> bool {
        match self {
            Self::Alive => true,
            Self::Dead => false,
        }
    }
}

impl From<bool> for WolframCodeState {
    fn from(value: bool) -> Self {
        if value {
            Self::Alive
        } else {
            Self::Dead
        }
    }
}
