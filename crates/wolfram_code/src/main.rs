use toolkit::{aliases::UniversalLatticePoint, lattice::universal_lattice::UniversalLattice};
use wolfram_code::{
    automaton::{WolframCodeAutomaton, WolframCodeLatticeSize},
    rule::WolframCodeRule,
    state::WolframCodeState,
};

use toolkit::prelude::*;

pub fn main() {
    let ca = WolframCodeAutomaton::new(WolframCodeRule::new(99));
    let mut l = UniversalLattice::<1, WolframCodeState>::from_states(
        vec![WolframCodeState::Dead],
        WolframCodeLatticeSize::new([10].to_vec()).into(),
    );

    let center = *l.size().get(0).unwrap() / 2;
    let len = *l.size().get(0).unwrap();

    l.set_state(&UniversalLatticePoint::new([center as i128]), &WolframCodeState::Alive);

    for _ in 0..30 {
        for x in 0..len {
            let state = l.get_state(&UniversalLatticePoint::new([x as i128]));

            let ch = match state {
                WolframCodeState::Alive => '█',
                WolframCodeState::Dead => '.',
            };

            print!("{ch}");
        }
        ca.step(&mut l);
        println!()
    }
}
