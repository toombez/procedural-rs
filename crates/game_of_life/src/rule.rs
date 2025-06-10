use toolkit::types::Rule;
use toolkit::prelude::*;

use crate::state::GameOfLifeState;
use crate::utils::{count_alive};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct GameOfLifeRule {
    amount_to_underpopulation: u8,
    amount_to_overpopulation: u8,
    amount_to_reproduction: u8,
}

impl Default for GameOfLifeRule {
    fn default() -> Self {
        Self {
            amount_to_underpopulation: 2,
            amount_to_overpopulation: 3,
            amount_to_reproduction: 3,
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl GameOfLifeRule {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(
        amount_to_underpopulation: u8,
        amount_to_overpopulation: u8,
        amount_to_reproduction: u8,
    ) -> Self {
        Self {
            amount_to_underpopulation,
            amount_to_overpopulation,
            amount_to_reproduction,
        }
    }
}

impl Rule for GameOfLifeRule {
    type State = GameOfLifeState;

    fn apply(
        &self,
        current_state: &Self::State,
        neighbors: &impl Neighborhood<State = Self::State>,
    ) -> Self::State {
        let alive_count = count_alive(neighbors) as u8;
        let is_alive = *current_state == GameOfLifeState::Alive;

        if is_alive {
            if alive_count < self.amount_to_underpopulation {
                return GameOfLifeState::Dead;
            }

            if alive_count > self.amount_to_overpopulation {
                return GameOfLifeState::Dead;
            }

            return GameOfLifeState::Alive;
        } else {
            if alive_count == self.amount_to_reproduction {
                return GameOfLifeState::Alive;
            }
        }

        return GameOfLifeState::Dead
    }
}
