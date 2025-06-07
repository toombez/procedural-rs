use crate::{rule::WolframCodeRule, state::WolframCodeState};
use toolkit::{
    lattice::lattice1::Lattice1, neighborhood::nearest::NearestNeighborhoodBuilder1,
    types::CellularAutomaton,
};

#[derive(Debug, Clone)]
pub struct WolframCodeAutomaton {
    rule: WolframCodeRule,
}

impl WolframCodeAutomaton {
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
