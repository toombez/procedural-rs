use crate::state::WolframCodeState;
use std::collections::HashMap;
use toolkit::types::Rule;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct WolframCodeRule {
    transform_map: HashMap<(bool, bool, bool), bool>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl WolframCodeRule {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(rule: u8) -> Self {
        let mut map = HashMap::new();

        for i in 0..8 {
            let result_bit = (rule >> i) & 1 != 0;

            let left = (i >> 2) & 1 != 0;
            let middle = (i >> 1) & 1 != 0;
            let right = (i >> 0) & 1 != 0;

            map.insert((left, middle, right), result_bit);
        }

        Self { transform_map: map }
    }
}

impl Rule for WolframCodeRule {
    type State = WolframCodeState;

    fn apply(
        &self,
        current_state: &Self::State,
        neighbors: &impl toolkit::prelude::Neighborhood<State = Self::State>,
    ) -> Self::State {
        let mut neighbors = neighbors.iter_states();

        let left = neighbors.nth(0).cloned().unwrap_or_default();
        let middle = current_state.clone();
        let right = neighbors.nth(0).cloned().unwrap_or_default();

        let new_state = self
            .transform_map
            .get(&(left.into(), middle.into(), right.into()))
            .cloned()
            .unwrap_or_default();

        WolframCodeState::from(new_state)
    }
}
