use crate::{rule::WolframCodeRule, state::WolframCodeState};
use toolkit::lattice::lattice1::Lattice1Size;
use toolkit::{
    lattice::{lattice1::Lattice1, lattice1_point::Lattice1Point},
    neighborhood::nearest::NearestNeighborhoodBuilder1,
    types::CellularAutomaton,
};

use lattice_wrapper_macros::prelude::*;

#[cfg(feature = "wasm")]
use toolkit::types::{BoundaryHandling, BoundaryHandlingLattice, Lattice};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct WolframCodeAutomaton {
    rule: WolframCodeRule,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl WolframCodeAutomaton {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(rule: WolframCodeRule) -> Self {
        Self { rule }
    }
}

impl CellularAutomaton for WolframCodeAutomaton {
    type Lattice = Lattice1<WolframCodeState>;
    type Rule = WolframCodeRule;
    type NeighborhoodBuilder = NearestNeighborhoodBuilder1;

    fn rule(&self) -> &Self::Rule {
        &self.rule
    }

    fn neighborhood_builder(&self) -> Self::NeighborhoodBuilder {
        NearestNeighborhoodBuilder1::new(1)
    }
}

#[cfg(feature = "wasm")]
type InnerLattice = Lattice1<WolframCodeState>;

#[cfg(feature = "wasm")]
lattice_wasm!(
    WolframCodeState,
    Lattice1Point,
    WolframCodeLattice,
    InnerLattice,
    Lattice1Size,
    WolframCodeAutomaton
);
