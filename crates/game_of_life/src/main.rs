use std::collections::HashMap;

use game_of_life::{automaton::GameOfLifeAutomaton, rule::GameOfLifeRule, state::GameOfLifeState};
use toolkit::{
    lattice::lattice2::{Lattice2, Lattice2Point},
    prelude::*,
};

pub fn main() {
    let ca = GameOfLifeAutomaton::new(GameOfLifeRule::default());

    let mut lattice2 = {
        let mut states = HashMap::new();

        for x in 0..80 {
            for y in 0..30 {
                states.insert(Lattice2Point::new(x, y), GameOfLifeState::Dead);
            }
        }

        // Glider
        states.insert(Lattice2Point::new(4, 0), GameOfLifeState::Alive);
        states.insert(Lattice2Point::new(4, 1), GameOfLifeState::Alive);
        states.insert(Lattice2Point::new(4, 2), GameOfLifeState::Alive);
        states.insert(Lattice2Point::new(3, 2), GameOfLifeState::Alive);
        states.insert(Lattice2Point::new(2, 1), GameOfLifeState::Alive);

        let mut states = Lattice2::from(states);
        states.set_boundary_handling(BoundaryHandling::Wrap);

        states
    };

    let size = lattice2.size();

    for _ in 0..100 {
        let mut s = String::new();

        for y in 0..size.1 {
            for x in 0..size.0 {
                let state = lattice2.get_state(&Lattice2Point::new(x as i128, y as i128));

                let ch = match state {
                    GameOfLifeState::Alive => "█",
                    GameOfLifeState::Dead => ".",
                };

                s += ch
            }
            s.push('\n');
        }

        print!("{s}");

        ca.step(&mut lattice2);
    }
}
