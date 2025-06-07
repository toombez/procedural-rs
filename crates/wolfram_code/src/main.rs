use toolkit::{
    lattice::lattice1::{Lattice1, Lattice1Point},
    prelude::*,
};
use wolfram_code::{
    automaton::WolframCodeAutomaton, rule::WolframCodeRule, state::WolframCodeState,
};

pub fn main() {
    let ca = WolframCodeAutomaton::new(WolframCodeRule::new(99));
    let mut l = Lattice1::from(vec![WolframCodeState::Dead; 80]);

    l.set_state(&Lattice1Point::new(l.size() as i128 / 2), &WolframCodeState::Alive);

    for _ in 0..30 {
        for x in 0..l.size() {
            let state = l.get_state(&Lattice1Point::new(x as i128));

            let ch = match state {
                WolframCodeState::Alive => '█',
                WolframCodeState::Dead => '.'
            };

            print!("{ch}");

        }
        ca.step(&mut l);
        println!()
    }
}
