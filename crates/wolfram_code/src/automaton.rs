use crate::{rule::WolframCodeRule, state::WolframCodeState};
#[cfg(feature = "wasm")]
use lattice_wrapper_macros::define_lattice_wrapper;
use lattice_wrapper_macros::{define_point_wrapper, define_size_wrapper};
use toolkit::{
    lattice::universal_lattice::UniversalLattice,
    neighborhood::nearest::NearestNeighborhoodBuilder1, types::CellularAutomaton,
};

#[cfg(feature = "wasm")]
use toolkit::{
    aliases::{UniversalLatticePoint, UniversalLatticeSize},
    types::{BoundaryHandling, BoundaryHandlingLattice, Lattice},
};

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

use toolkit::prelude::*;

impl CellularAutomaton for WolframCodeAutomaton {
    type Lattice = UniversalLattice<1, WolframCodeState>;
    type Rule = WolframCodeRule;
    type NeighborhoodBuilder = NearestNeighborhoodBuilder1;

    fn rule(&self) -> &Self::Rule {
        &self.rule
    }

    fn neighborhood_builder(&self) -> Self::NeighborhoodBuilder {
        NearestNeighborhoodBuilder1::new(1)
    }

    fn step(&self, lattice: &mut Self::Lattice) {
        let points = lattice.points();

        let mut new_states = Vec::with_capacity(points.len());
        let rule = self.rule();
        let builder = self.neighborhood_builder();

        for point in &points {
            let neighborhood = builder.build_neighborhood(&point, lattice);
            let current_state = lattice.get_state(&point);

            let new_state = rule.apply(&current_state, &neighborhood);
            new_states.push(new_state);
        }

        for (point, new_state) in points.into_iter().zip(new_states) {
            lattice.set_state(&point, &new_state);
        }
    }
}

#[cfg(feature = "wasm")]
type InnerSize = UniversalLatticeSize<1>;
#[cfg(feature = "wasm")]
define_size_wrapper!(WolframCodeLatticeSize, InnerSize);

#[cfg(feature = "wasm")]
type InnerPoint = UniversalLatticePoint<1>;
#[cfg(feature = "wasm")]
define_point_wrapper!(WolframCodeLatticePoint, InnerPoint);

#[cfg(feature = "wasm")]
type InnerLattice = UniversalLattice<1, WolframCodeState>;
#[cfg(feature = "wasm")]
define_lattice_wrapper!(
    WolframCodeLattice,
    WolframCodeState,
    WolframCodeLatticePoint,
    WolframCodeLatticeSize,
    WolframCodeAutomaton,
    InnerLattice,
    InnerSize,
    InnerPoint
);
