use lattice_wrapper_macros::prelude::*;
#[cfg(feature = "wasm")]
#[cfg(feature = "wasm")]
use toolkit::lattice::lattice2::Lattice2Size;
#[cfg(feature = "wasm")]
use toolkit::lattice::lattice2_point::Lattice2Point;
use toolkit::{
    lattice::lattice2::Lattice2, neighborhood::moore::MooreNeighborhoodBuilder,
    types::CellularAutomaton,
};

#[cfg(feature = "wasm")]
use toolkit::types::BoundaryHandlingLattice;
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

#[cfg(feature = "wasm")]
type InnerLattice = Lattice2<GameOfLifeState>;

#[cfg(feature = "wasm")]
lattice_wasm!(
    GameOfLifeState,
    Lattice2Point,
    GameOfLifeLattice,
    InnerLattice,
    Lattice2Size,
    GameOfLifeAutomaton
);
