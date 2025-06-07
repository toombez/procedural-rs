use toolkit::{
    lattice::lattice2::Lattice2, neighborhood::moore::MooreNeighborhoodBuilder,
    types::CellularAutomaton,
};

use crate::{rule::GameOfLifeRule, state::GameOfLifeState};
pub struct GameOfLifeAutomaton {
    rule: GameOfLifeRule,
}

impl GameOfLifeAutomaton {
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
