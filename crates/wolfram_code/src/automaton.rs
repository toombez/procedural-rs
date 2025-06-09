use crate::{rule::WolframCodeRule, state::WolframCodeState};
use toolkit::{
    lattice::lattice1::Lattice1,
    neighborhood::nearest::NearestNeighborhoodBuilder1,
    types::{CellularAutomaton},
};

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
#[wasm_bindgen(js_name = "WolframCodeLattice")]
#[derive(Debug, Clone)]
pub struct WasmWolframCodeLattice {
    lattice: Lattice1<WolframCodeState>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_class = "WolframCodeLattice")]
impl WasmWolframCodeLattice {
    #[wasm_bindgen(constructor)]
    pub fn new(points: Vec<WolframCodeState>) -> Self {
        Self {
            lattice: Lattice1::<WolframCodeState>::from(points),
        }
    }

    #[wasm_bindgen]
    pub fn set_boundary_handing(&mut self, boundary_handling: BoundaryHandling) {
        self.lattice.set_boundary_handling(boundary_handling);
    }

    #[wasm_bindgen]
    pub fn states(&self) -> Vec<WolframCodeState> {
        self.lattice
            .points()
            .iter()
            .map(|point| self.lattice.get_state(point))
            .collect()
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WolframCodeAutomaton {
    #[wasm_bindgen(js_name = "step")]
    pub fn wasm_step(&self, lattice: &mut WasmWolframCodeLattice) {
        self.step(&mut lattice.lattice);
    }
}
