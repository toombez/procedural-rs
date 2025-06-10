use toolkit::{
    lattice::lattice2::Lattice2, neighborhood::moore::MooreNeighborhoodBuilder,
    types::CellularAutomaton,
};

use std::collections::HashMap;

#[cfg(feature = "wasm")]
use toolkit::{lattice::lattice2::Lattice2Point};
#[cfg(feature = "wasm")]
use toolkit::types::Lattice;

use crate::{rule::GameOfLifeRule, state::GameOfLifeState};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct GameOfLifeAutomaton {
    rule: GameOfLifeRule,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl GameOfLifeAutomaton {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(rule: GameOfLifeRule) -> Self {
        GameOfLifeAutomaton { rule }
    }
}

impl CellularAutomaton for GameOfLifeAutomaton {
    type Lattice = Lattice2<GameOfLifeState>;
    type Rule = GameOfLifeRule;
    type NeighborhoodBuilder = MooreNeighborhoodBuilder;

    fn rule(&self) -> &Self::Rule {
        &self.rule
    }

    fn neighborhood_builder(&self) -> Self::NeighborhoodBuilder {
        MooreNeighborhoodBuilder
    }
}

#[derive(Debug, Clone)]
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct GameOfLifeLattice(Lattice2<GameOfLifeState>);

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl GameOfLifeLattice {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Lattice2::from(HashMap::new()))
    }

    #[wasm_bindgen]
    pub fn set_state(&mut self, point: &Lattice2Point, state: GameOfLifeState) {
        self.0.set_state(&point, &state);
        self.0.calculate_size();
    }

    #[wasm_bindgen]
    pub fn get_state(&self, point: &Lattice2Point) -> GameOfLifeState {
        self.0.get_state(&point)
    }

    #[wasm_bindgen(getter)]
    pub fn states(&self) -> Vec<GameOfLifeState> {
        self.0
            .points()
            .iter()
            .map(|point| self.get_state(point))
            .collect()
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl GameOfLifeAutomaton {
    #[wasm_bindgen(js_name = "step")]
    pub fn wasm_step(&self, lattice: &mut GameOfLifeLattice) {
        self.step(&mut lattice.0);
    }
}
