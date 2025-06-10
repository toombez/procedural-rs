use crate::{rule::WolframCodeRule, state::WolframCodeState};
use toolkit::{
    lattice::lattice1::Lattice1,
    neighborhood::nearest::NearestNeighborhoodBuilder1,
    types::{CellularAutomaton},
};

#[cfg(feature = "wasm")]
use toolkit::{lattice::lattice1::Lattice1Point, types::{BoundaryHandling, BoundaryHandlingLattice, Lattice}};

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
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WolframCodeLattice(Lattice1<WolframCodeState>);

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WolframCodeLattice {
    #[wasm_bindgen(constructor)]
    pub fn new(points: Vec<WolframCodeState>) -> Self {
        Self(Lattice1::<WolframCodeState>::from(points))
    }

    #[wasm_bindgen]
    pub fn from_size(size: usize) -> Self {
        Self(Lattice1::<WolframCodeState>::from(size))
    }

    #[wasm_bindgen]
    pub fn set_boundary_handing(&mut self, boundary_handling: BoundaryHandling) {
        self.0.set_boundary_handling(boundary_handling);
    }

    #[wasm_bindgen]
    pub fn set_state(&mut self, point: &Lattice1Point, state: WolframCodeState) {
        self.0.set_state(point, &state);
    }

    #[wasm_bindgen]
    pub fn get_state(&self, point: &Lattice1Point) -> WolframCodeState {
        self.0.get_state(point)
    }

    #[wasm_bindgen(getter)]
    pub fn points(&self) -> Vec<Lattice1Point> {
        self.0.points()
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> usize {
        self.0.size()
    }

    #[wasm_bindgen(setter, js_name = "size")]
    pub fn set_size(&mut self, size: usize) {
        self.0.set_size(size);
    }

    #[wasm_bindgen(getter)]
    pub fn states(&self) -> Vec<WolframCodeState> {
        self.0
            .points()
            .iter()
            .map(|point| self.get_state(point))
            .collect()
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WolframCodeAutomaton {
    #[wasm_bindgen(js_name = "step")]
    pub fn wasm_step(&self, lattice: &mut WolframCodeLattice) {
        self.step(&mut lattice.0);
    }
}
