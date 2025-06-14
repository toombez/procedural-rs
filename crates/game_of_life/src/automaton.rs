use lattice_wrapper_macros::define_lattice_wrapper;
use lattice_wrapper_macros::define_point_wrapper;
use lattice_wrapper_macros::define_size_wrapper;

#[cfg(feature = "wasm")]
use toolkit::lattice::universal_lattice::UniversalLattice;
#[cfg(feature = "wasm")]
use toolkit::lattice::universal_lattice_point::UniversalLatticePoint;
#[cfg(feature = "wasm")]
use toolkit::lattice::universal_lattice_size::UniversalLatticeSize;

use toolkit::neighborhood::moore::MooreNeighborhoodBuilder;
#[cfg(feature = "wasm")]
#[cfg(feature = "wasm")]
use toolkit::types::BoundaryHandling;
use toolkit::{
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
    type Lattice = UniversalLattice<2, GameOfLifeState>;
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
type InnerSize = UniversalLatticeSize<2>;
#[cfg(feature = "wasm")]
define_size_wrapper!(
    GameOfLifeLatticeSize,
    InnerSize
);

#[cfg(feature = "wasm")]
type InnerPoint = UniversalLatticePoint<2>;
#[cfg(feature = "wasm")]
define_point_wrapper!(
    GameOfLifeLatticePoint,
    InnerPoint
);

#[cfg(feature = "wasm")]
type InnerLattice = UniversalLattice<2, GameOfLifeState>;
#[cfg(feature = "wasm")]
define_lattice_wrapper!(
    GameOfLifeLattice,
    GameOfLifeLatticePoint,
    GameOfLifeState,
    GameOfLifeLatticeSize,
    InnerLattice,
    GameOfLifeAutomaton,
    InnerSize,
    InnerPoint
);
